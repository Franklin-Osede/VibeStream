
import fs from 'fs/promises';
import path from 'path';
import { ProjectState, INITIAL_STATE } from './types.js';

const STATE_FILE = 'project_state.json';

export class Store {
  private state: ProjectState;
  private basePath: string;

  constructor(basePath: string) {
    this.basePath = basePath;
    this.state = INITIAL_STATE;
  }

  async init() {
    try {
      const data = await fs.readFile(path.join(this.basePath, STATE_FILE), 'utf-8');
      this.state = JSON.parse(data);
    } catch (error) {
      // If file doesn't exist, use initial state and save it
      await this.save();
    }
  }

  getState(): ProjectState {
    return this.state;
  }

  async update(updater: (state: ProjectState) => void) {
    updater(this.state);
    await this.save();
  }

  private async save() {
    await fs.writeFile(
      path.join(this.basePath, STATE_FILE),
      JSON.stringify(this.state, null, 2),
      'utf-8'
    );
  }
}
