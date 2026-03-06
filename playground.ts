#!/usr/bin/env node
import { Airtable } from "./typescript/output";

async function main() {
	console.log("Hello World (TypeScript)");

	const airtable = new Airtable();
	const p = await airtable.primary.get("recNk6Lmrr5y3Fx81");
	console.log(airtable.url());
	console.log(airtable.primary.url());
	console.log(airtable.primary.url("Grid view"));
	console.log(p.URL());
	console.log(p.URL("Grid view"));
}

main().catch((err) => {
	console.error("Error in main:", err);
	process.exit(1);
});
