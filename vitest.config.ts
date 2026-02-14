import { defineConfig } from "vitest/config";
import { loadEnv } from "vite";

export default defineConfig({
	test: {
		include: ["**/*.test.ts", "**/*.test.js"],
		env: loadEnv("", process.cwd(), ""),
		testTimeout: 30_000,
	},
});
