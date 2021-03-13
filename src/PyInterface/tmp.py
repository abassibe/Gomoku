In function exit_pool, I wait until threads are finished and delete the QThreadPool instance...

Is there a way not to wait for each thread and stop everything directly ?

EDIT Solution:

So I have approached the subject differently. I replaced my QRunnable with a QThread. I removed QThreadPool and I manage threads myself in a list. I also added a pyqtSignal in order to stop the QTimer and close the running threads by quit() function.

Like that all my thread quit without problem.

import sys
from PyQt5.Qt import QThread, QApplication, QWidget, QVBoxLayout
from PyQt5.Qt import QTimer, QObject, QPushButton, QLabel, pyqtSignal


class BackendQThread(QThread):
    """
        Class who create a QThread to trigger requests
    """

    quit_thread = pyqtSignal(name='close_thread')

    def __init__(self, task):
        super(BackendQThread, self).__init__()
        self.task = task

    def run(self):
        """
        Run the actions depending on the selected task

        """

        # Here I make long requests
        if 'user' in self.task:
            self.query_user_data()
        elif 'host' in self.task:
            self.query_hosts_data()
        elif 'service' in self.task:
            self.query_services_data()
        elif 'alignakdaemon' in self.task:
            self.query_daemons_data()
        elif 'livesynthesis' in self.task:
            self.query_livesynthesis_data()
        elif 'history' in self.task:
            self.query_history_data()
        elif 'notifications' in self.task:
            self.query_notifications_data()
        else:
            pass

    @staticmethod
    def query_user_data():
        """
        Launch request for "user" endpoint

        """

        print('Query user data')

    @staticmethod
    def query_hosts_data():
        """
        Launch request for "host" endpoint

        """

        print('Query hosts')

    @staticmethod
    def query_services_data():
        """
        Launch request for "service" endpoint

        """

        print("Query services")

    @staticmethod
    def query_daemons_data():
        """
        Launch request for "alignakdaemon" endpoint

        """

        print('Query daemons')

    @staticmethod
    def query_livesynthesis_data():
        """
        Launch request for "livesynthesis" endpoint

        """

        print('query livesynthesis')

    @staticmethod
    def query_history_data():
        """
        Launch request for "history" endpoint but only for hosts in "data_manager"

        """

        print('Query history')

    @staticmethod
    def query_notifications_data():
        """
        Launch request for "history" endpoint but only for notifications of current user

        """

        print('Query notifications')


class ThreadManager(QObject):
    """
        Class who create BackendQThread to periodically request on a Backend
    """

    def __init__(self, parent=None):
        super(ThreadManager, self).__init__(parent)
        self.tasks = self.get_tasks()
        self.timer = QTimer()
        self.threads = []

    def start(self):
        """
        Start ThreadManager

        """

        print("Start backend Manager...")

        # Make a first request
        self.create_tasks()

        # Then request periodically
        self.timer.setInterval(10000)
        self.timer.start()
        self.timer.timeout.connect(self.create_tasks)

    @staticmethod
    def get_tasks():
        """
        Return the available tasks to run

        :return: tasks to run
        :rtype: list
        """

        return [
            'notifications', 'livesynthesis', 'alignakdaemon', 'history', 'service', 'host', 'user',
        ]

    def create_tasks(self):
        """
        Create tasks to run

        """

        # Here I reset the list of threads
        self.threads = []
        for cur_task in self.tasks:
            backend_thread = BackendQThread(cur_task)

            # Add task to QThreadPool
            backend_thread.start()
            self.threads.append(backend_thread)

    def stop(self):
        """
        Stop the manager and close all QThreads

        """

        print("Stop tasks")
        self.timer.stop()
        for task in self.threads:
            task.quit_thread.emit()

        print("Tasks finished")


if __name__ == '__main__':
    app = QApplication(sys.argv)

    layout = QVBoxLayout()
    widget = QWidget()
    widget.setLayout(layout)

    thread_manager = ThreadManager()

    start_btn = QPushButton("Start")
    start_btn.clicked.connect(thread_manager.start)
    layout.addWidget(start_btn)

    stop_btn = QPushButton("Stop")
    stop_btn.clicked.connect(thread_manager.stop)
    layout.addWidget(stop_btn)

    widget.show()

    sys.exit(app.exec_())

