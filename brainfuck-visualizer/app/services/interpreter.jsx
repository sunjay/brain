const {spawn} = require('child_process');
const fs = require('fs');

const {
  ACTION_START,
  appendOutput,
  appendError,
  addHistory,
  setSource,
} = require('../actions/InterpreterActions');

const actionHandlers = {
  [ACTION_START]({command, file}, {dispatch}) {
    command = command.split(/\s+/);
    command.push(file);

    fs.readFile(file, (err, source) => {
      if (err) throw err;

      dispatch(setSource(source));
      this.spawnWorker(command);
    });
  },
};

class Interpreter {
  constructor() {
    this.worker = null;
    this.spawnWorker = null;
    this.queue = [''];
  }

  start({dispatch}) {
    // Lazily load the worker since not every process that starts this server
    // will need this process right away or at all
    this.spawnWorker = ([command, ...args]) => {
      console.info('Spawning interpreter');
      this.worker = spawn(command, args);

      this.worker.stderr.on('data', (data) => {
        // The data that comes in is not guarenteed to be a complete command line
        // To account for this, we maintain a queue of commands since we
        // may get several complete commands or no complete commands in this data
        // The final element in the queue always contains the latest incomplete
        // response
        // When a response is completed with the \n, this split call will result
        // in an empty string at the end because 'abc\n'.split('\n') == ['abc', '']
        // That empty string at the end will become the next latest incomplete
        // response
        // Even if we get something like 'abc\nfoo', foo is incomplete because
        // it has not been terminated with a newline. Thus, it will remain at the
        // end of the queue and keep being appended to until it is complete
        const lines = data.toString().split('\n');
        this.queue[this.queue.length - 1] += lines[0];
        this.queue.push(...lines.slice(1));

        for (const line of this.queue.slice(0, -1)) {
          this.processResponse(dispatch, line);
        }
        this.queue = this.queue.slice(this.queue.length - 1);
      });

      this.worker.stdout.on('data', (data) => {
        console.info(`interpreter stdout: ${data}`);
        dispatch(appendOutput(data.toString()));
      });

      this.worker.on('close', (code) => {
        console.info(`child process exited with code ${code}`);
      });

      this.worker.on('disconnect', () => {
        console.warn('child process was disconnected');
      });

      this.worker.on('error', (error) => {
        console.error(`interpreter error: ${error}`);
      });
    };
  }

  processResponse(dispatch, data) {
    let response;
    try {
      response = JSON.parse(data);
    }
    catch (e) {
      dispatch(appendError(data));
      return;
    }

    dispatch(addHistory({
      ...response,
      memory: response.memory.trim().split(/\s+/).map(parseFloat),
    }));
  }

  middleware() {
    return (store) => (next) => (action) => {
      const handler = actionHandlers[action.type];
      if (handler) {
        handler.call(this, action, store);
      }

      next(action);
    };
  }

  send(message) {
    if (!this.worker) {
      throw new Error('Worker not spawned before message sent out');
    }
    console.info('Sending to interpreter', message);
    this.worker.stdin.write(JSON.stringify(message) + '\n');
  }
}

module.exports = Interpreter;
