
export type Phase = 'BACKEND' | 'FRONTEND';

export interface EndpointDef {
  id: string;
  method: string;
  path: string;
  description: string;
  parameters?: string[];
}

export interface BackendTask {
  id: string;
  description: string;
  status: 'PENDING' | 'IN_PROGRESS' | 'COMPLETED';
  endpointDef?: EndpointDef;
}

export interface Screen {
  id: string;
  name: string;
  description: string;
  status: 'DESIGNED' | 'HTML_BUILT' | 'CONNECTED' | 'COMPLETED';
  requiredEndpoints: string[]; // IDs of BackendTasks (endpoints)
}

export interface ProjectState {
  currentProject: string;
  phase: Phase;
  backend: {
    completed: boolean;
    tasks: BackendTask[];
  };
  frontend: {
    completed: boolean;
    screens: Screen[];
  };
}

export const INITIAL_STATE: ProjectState = {
  currentProject: 'vibestream-core',
  phase: 'BACKEND',
  backend: {
    completed: false,
    tasks: []
  },
  frontend: {
    completed: false,
    screens: []
  }
};
