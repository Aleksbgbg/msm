import { invoke } from "./invoke";

export interface FirstTimeSetup {
  required: boolean;
  hasJava: boolean;
}

export function requiresFirstTimeSetup(): Promise<FirstTimeSetup2> {
  return invoke("requires_first_time_setup");
}
