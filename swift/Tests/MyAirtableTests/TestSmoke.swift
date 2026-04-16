// Smoke test — verifies the generated `MyAirtable` module links. Replaced
// with real integration tests in F3 onward (myairtable-793, etc.).

import MyAirtable
import Testing

@Suite("Smoke")
struct TestSmoke {
    @Test("Generated module imports")
    func testModuleImports() {
        // If this file compiles, the generated package linked successfully.
        #expect(true)
    }
}
