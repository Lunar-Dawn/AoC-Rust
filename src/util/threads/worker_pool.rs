use crate::util::threads::task_queue::TaskQueue;
use std::any::Any;
use std::ops::DerefMut;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub struct WorkerPool<InputType, Accumulator> {
    handles: Vec<JoinHandle<Accumulator>>,

    queue: Arc<Mutex<TaskQueue<InputType>>>,
    condvar: Arc<Condvar>,
}

impl<InputType, Accumulator> WorkerPool<InputType, Accumulator>
where
    InputType: Send + 'static,
    Accumulator: Clone + Send + 'static,
{
    pub fn new<Runner>(num_treads: usize, default_value: Accumulator, runner: Runner) -> Self
    where
        Runner: Fn(InputType, &mut Accumulator) -> () + Clone + Send + 'static,
    {
        let mut handles = Vec::with_capacity(num_treads);

        let condvar = Arc::new(Condvar::new());
        let queue = Arc::new(Mutex::new(TaskQueue::new()));

        for _ in 0..num_treads {
            handles.push(Self::spawn_worker(
                queue.clone(),
                condvar.clone(),
                default_value.clone(),
                runner.clone(),
            ));
        }

        WorkerPool {
            handles,
            condvar,
            queue,
        }
    }

    pub fn push(&mut self, input: InputType) {
        self.queue.lock().unwrap().push(input);
        self.condvar.notify_one();
    }

    pub fn stop(
        &mut self,
    ) -> impl Iterator<Item = Result<Accumulator, Box<dyn Any + Send>>> + use<'_, InputType, Accumulator>
    {
        self.queue.lock().unwrap().stop();
        self.condvar.notify_all();

        self.handles.drain(..).map(|h| h.join())
    }
    pub fn kill(
        &mut self,
    ) -> impl Iterator<Item = Result<Accumulator, Box<dyn Any + Send>>> + use<'_, InputType, Accumulator>
    {
        self.queue.lock().unwrap().kill();
        self.condvar.notify_all();

        self.handles.drain(..).map(|h| h.join())
    }

    fn spawn_worker<Runner>(
        queue: Arc<Mutex<TaskQueue<InputType>>>,
        condvar: Arc<Condvar>,
        default_value: Accumulator,
        f: Runner,
    ) -> JoinHandle<Accumulator>
    where
        Runner: Fn(InputType, &mut Accumulator) -> () + Clone + Send + 'static,
    {
        thread::spawn(move || Self::worker(condvar, queue, default_value, f))
    }

    fn worker(
        condvar: Arc<Condvar>,
        queue: Arc<Mutex<TaskQueue<InputType>>>,
        default_value: Accumulator,
        runner: impl Fn(InputType, &mut Accumulator) -> (),
    ) -> Accumulator {
        let mut result = default_value;
        loop {
            match Self::receive(&queue, &condvar) {
                Some(input) => runner(input, &mut result),
                None => return result,
            }
        }
    }
    fn receive(
        queue: &Arc<Mutex<TaskQueue<InputType>>>,
        condvar: &Arc<Condvar>,
    ) -> Option<InputType> {
        let mut guard = condvar
            .wait_while(queue.lock().unwrap(), |q| !q.can_pop())
            .unwrap();
        let queue = guard.deref_mut();

        queue.pop()
    }
}
