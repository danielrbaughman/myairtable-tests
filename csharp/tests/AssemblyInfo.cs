using Xunit;

// Integration tests hit a shared live Airtable base, so cross-class parallelism is OFF
// (the xUnit v2 mechanism is this assembly attribute — NOT xunit.runner.json, which is v3).
// Within a class, sequential CRUD phases are ordered via [TestPriority] + PriorityOrderer.
[assembly: CollectionBehavior(DisableTestParallelization = true)]
