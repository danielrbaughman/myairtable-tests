using Xunit.Abstractions;
using Xunit.Sdk;

namespace MyAirtable.Tests;

/// <summary>
/// Orders the test methods of a class by an explicit priority. The C# analog of the
/// JVM targets' @TestMethodOrder(OrderAnnotation) — needed because xUnit has no built-in
/// method ordering and instantiates a new test class per method. Apply with
/// <c>[TestCaseOrderer(PriorityOrderer.TypeName, PriorityOrderer.AssemblyName)]</c> on a
/// CRUD test class whose phases (create -> read -> update -> delete) share an IClassFixture.
/// </summary>
public sealed class PriorityOrderer : ITestCaseOrderer
{
    public const string TypeName = "MyAirtable.Tests.PriorityOrderer";
    public const string AssemblyName = "CSharpTests";

    public IEnumerable<TTestCase> OrderTestCases<TTestCase>(IEnumerable<TTestCase> testCases)
        where TTestCase : ITestCase
    {
        var assemblyName = typeof(TestPriorityAttribute).AssemblyQualifiedName!;
        return testCases
            .Select(tc =>
            {
                var attr = tc.TestMethod.Method.GetCustomAttributes(assemblyName).FirstOrDefault();
                var priority =
                    attr?.GetNamedArgument<int>(nameof(TestPriorityAttribute.Priority)) ?? 0;
                return (tc, priority);
            })
            .OrderBy(x => x.priority)
            .Select(x => x.tc);
    }
}

/// <summary>Assigns an ordering priority to a test method (lower runs first).</summary>
[AttributeUsage(AttributeTargets.Method)]
public sealed class TestPriorityAttribute : Attribute
{
    public TestPriorityAttribute(int priority) => Priority = priority;

    public int Priority { get; }
}
