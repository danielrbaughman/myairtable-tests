using Xunit;

namespace MyAirtable.Tests;

/// <summary>
/// Smoke test for the test harness itself: proves the sequenced-CRUD infrastructure works
/// end-to-end (IClassFixture shared state + PriorityOrderer method ordering + TestPriority)
/// before any generated client exists. F3 upgrades this to also assert
/// <c>MyAirtableRuntimeInfo.Version</c> is reachable from the generated output.
/// </summary>
[TestCaseOrderer(PriorityOrderer.TypeName, PriorityOrderer.AssemblyName)]
public class TestHarness : IClassFixture<CrudFixture>
{
    private readonly CrudFixture _fixture;

    public TestHarness(CrudFixture fixture) => _fixture = fixture;

    [Fact]
    [TestPriority(1)]
    public void Phase1_RecordsSharedState() => _fixture.CreatedIds["smoke"] = "rec1";

    [Fact]
    [TestPriority(2)]
    public void Phase2_SeesEarlierPhaseState() =>
        Assert.Equal("rec1", _fixture.CreatedIds["smoke"]);
}
