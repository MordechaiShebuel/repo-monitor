type Query {
  podmanRepoStatus: ServiceStatus!
  buildServerStatus: BuildServerStatus!
  managedApps: [ManagedApp!]!
  buildJob(id: ID!): BuildJob
}

type Mutation {
  changeAppVersion(appId: ID!, version: String!): ManagedApp!
  restartPodmanRepo: ActionResult!
  runBuild: BuildJob!
}

type ServiceStatus {
  state: ServiceState!
  checkedAt: String!
  message: String
}

type BuildServerStatus {
  state: BuildServerState!
  checkedAt: String!
  currentJobId: ID
}

type ManagedApp {
  id: ID!
  name: String!
  endpoint: String
  version: String
}

type BuildJob {
  id: ID!
  state: JobState!
  startedAt: String
  finishedAt: String
  message: String
}

type ActionResult {
  accepted: Boolean!
  message: String!
}

enum ServiceState { ACTIVE DOWN UNKNOWN }
enum BuildServerState { ACTIVE IDLE DOWN UNKNOWN }
enum JobState { QUEUED RUNNING SUCCEEDED FAILED }
