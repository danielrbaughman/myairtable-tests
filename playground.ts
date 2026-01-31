#!/usr/bin/env node
import { Airtable } from "./typescript/output";

async function main() {
	console.log("Hello World (TypeScript)");

	const airtable = new Airtable();
	const p = await airtable.primary.get("recNk6Lmrr5y3Fx81", { returnAs: "interface" });
	console.log(p.fields["User"]);
}

main().catch((err) => {
	console.error("Error in main:", err);
	process.exit(1);
});
