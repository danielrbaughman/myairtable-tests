// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

const { getApiKey, getBaseId, setAirtableConfig } = require("../static/helpers");
const { FormulasTable, PrimaryTable, SecondaryTable, TertiaryTable } = require("./tables");
const { TableNamePropertyMapping } = require("./types");

/** Airtable base wrapper */
class Airtable {
	/** `Formulas` (tblnuYBsMdXNDsuRc) */
	formulas;
	/** `Primary` (tblmb3iqgpNS1ysV2) */
	primary;
	/** `Secondary` (tblPPScS3XMuFkDYN) */
	secondary;
	/** `Tertiary` (tblLFoLxEdWlxjmLP) */
	tertiary;

	constructor(options = {}) {
		const _baseId = options.baseId || getBaseId();
		const _options = {
			apiKey: options.apiKey ?? getApiKey(),
			apiVersion: options.apiVersion,
			customHeaders: options.customHeaders,
			endpointUrl: options.endpointUrl,
			noRetryIfRateLimited: options.noRetryIfRateLimited ?? false,
			requestTimeout: options.requestTimeout,
		};
		setAirtableConfig(_baseId, _options);
		this.formulas = new FormulasTable(_baseId, _options);
		this.primary = new PrimaryTable(_baseId, _options);
		this.secondary = new SecondaryTable(_baseId, _options);
		this.tertiary = new TertiaryTable(_baseId, _options);
	}

	/** Get a table by its Airtable name. */
	table(tableName) {
		return this[TableNamePropertyMapping[tableName]];
	}
}

module.exports = { Airtable };
