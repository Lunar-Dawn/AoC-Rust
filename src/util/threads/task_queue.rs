use std::collections::VecDeque;

enum QueueStatus<InputType> {
    Active(VecDeque<InputType>),
    Stopping(VecDeque<InputType>),
    Killed,
}

pub struct TaskQueue<InputType> {
    state: QueueStatus<InputType>,
}
impl<InputType> TaskQueue<InputType> {
    pub fn new() -> Self {
        TaskQueue {
            state: QueueStatus::Active(VecDeque::new()),
        }
    }

    pub fn push(&mut self, input: InputType) {
        match &mut self.state {
            QueueStatus::Active(queue) => queue.push_back(input),
            QueueStatus::Stopping(_) => panic!("Tried to push work to pool shutting down"),
            QueueStatus::Killed => panic!("Tried to push work to shutdown pool"),
        }
    }
    pub fn pop(&mut self) -> Option<InputType> {
        match &mut self.state {
            QueueStatus::Active(queue) => queue.pop_front(),
            QueueStatus::Stopping(queue) => {
                let ret = queue.pop_front();
                if queue.is_empty() {
                    self.state = QueueStatus::Killed;
                }
                ret
            }
            QueueStatus::Killed => None,
        }
    }
    pub fn can_pop(&self) -> bool {
        match &self.state {
            QueueStatus::Active(queue) => !queue.is_empty(),
            QueueStatus::Stopping(_) | QueueStatus::Killed => true,
        }
    }

    pub fn stop(&mut self) {
        match &mut self.state {
            QueueStatus::Active(queue) => {
                let queue = std::mem::take(queue);
                self.state = QueueStatus::Stopping(queue)
            }
            _ => (),
        };
    }
    pub fn kill(&mut self) {
        self.state = QueueStatus::Killed;
    }
}
