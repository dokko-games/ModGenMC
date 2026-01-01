export interface Version {
  major: number;
  minor: number;
  patch: number;
}

export interface Project {
  name: string;
  path: string;
  targetVersion: Version;
}
