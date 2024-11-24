# QueueManager.py


class QueueManager:
    def __init__(self, path):
        self.path = path

    def handle_overflow(self):
        raise NotImplementedError

    def set_drop_strategy(self):
        raise NotImplementedError
