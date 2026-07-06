// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

const {
    getApiKey,
    getBaseId,
    setAirtableConfig,
    buildUrl,
} = require("../static/helpers");
const {
    FormulasTable,
    PrimaryTable,
    SecondaryTable,
    TertiaryTable,
} = require("./tables");
const { TableNamePropertyMapping } = require("./types");

/** Airtable base wrapper */
class Airtable {
    baseId;

    /** `Formulas` (tblnuYBsMdXNDsuRc) */
    formulas;
    /** `Primary` (tblmb3iqgpNS1ysV2) */
    primary;
    /** `Secondary` (tblPPScS3XMuFkDYN) */
    secondary;
    /** `Tertiary` (tblLFoLxEdWlxjmLP) */
    tertiary;

    constructor(options = {}) {
        this.baseId = options.baseId || getBaseId();
        const _options = {
            apiKey: options.apiKey ?? getApiKey(),
            apiVersion: options.apiVersion,
            customHeaders: options.customHeaders,
            endpointUrl: options.endpointUrl,
            noRetryIfRateLimited: options.noRetryIfRateLimited ?? false,
            requestTimeout: options.requestTimeout,
            cacheSeconds: options.cacheSeconds,
        };
        setAirtableConfig(this.baseId, _options);
        this.formulas = new FormulasTable(this.baseId, _options);
        this.primary = new PrimaryTable(this.baseId, _options);
        this.secondary = new SecondaryTable(this.baseId, _options);
        this.tertiary = new TertiaryTable(this.baseId, _options);
    }

    /** Get a table by its Airtable name. */
    table(tableName) {
        return this[TableNamePropertyMapping[tableName]];
    }

    /** Get the URL for the Airtable base. */
    url() {
        return buildUrl(this.baseId);
    }

    /** Fetch a live version of the schema from Airtable's metadata API. */
    async getSchema() {
        const url = `https://api.airtable.com/v0/meta/bases/${this.baseId}/tables`;
        const response = await fetch(url, {
            headers: { Authorization: `Bearer ${getApiKey(this.baseId)}` },
        });
        if (!response.ok) {
            throw new Error(`Failed to fetch schema: ${response.status} ${response.statusText}`);
        }
        return response.json();
    }

    /** Invalidates the cache for all tables. */
    invalidateCache() {
        this.formulas.invalidateCache();
        this.primary.invalidateCache();
        this.secondary.invalidateCache();
        this.tertiary.invalidateCache();
    }
}

module.exports = { Airtable };
