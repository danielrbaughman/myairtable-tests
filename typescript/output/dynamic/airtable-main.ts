// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

import { ExtendedAirtableOptions } from "../static/special-types";
import { BaseSchema } from "../static/schema-types";
import {
    getApiKey,
    getBaseId,
    setAirtableConfig,
    buildUrl,
} from "../static/helpers";
import {
    FormulasTable,
    PrimaryTable,
    SecondaryTable,
    TertiaryTable,
    TableNameToTableType,
} from "./tables";
import { TableName, TableNamePropertyMapping } from "./types";

    /** Airtable base wrapper */
export class Airtable {
    public baseId: string;

    /** `Formulas` (tblnuYBsMdXNDsuRc) */
    public formulas: FormulasTable;
    /** `Primary` (tblmb3iqgpNS1ysV2) */
    public primary: PrimaryTable;
    /** `Secondary` (tblPPScS3XMuFkDYN) */
    public secondary: SecondaryTable;
    /** `Tertiary` (tblLFoLxEdWlxjmLP) */
    public tertiary: TertiaryTable;

    constructor(options?: ExtendedAirtableOptions) {
        this.baseId = options?.baseId || getBaseId();
        const _options = {
              apiKey: options?.apiKey ?? getApiKey(),
              apiVersion: options?.apiVersion,
              customHeaders: options?.customHeaders,
              endpointUrl: options?.endpointUrl,
              noRetryIfRateLimited: options?.noRetryIfRateLimited ?? false,
              requestTimeout: options?.requestTimeout,
              cacheSeconds: options?.cacheSeconds,
        };
        setAirtableConfig(this.baseId, _options);
        this.formulas = new FormulasTable(this.baseId, _options);
        this.primary = new PrimaryTable(this.baseId, _options);
        this.secondary = new SecondaryTable(this.baseId, _options);
        this.tertiary = new TertiaryTable(this.baseId, _options);
    }


    /** Get a table by its Airtable name. */
    public table<T extends TableName>(tableName: T): TableNameToTableType[T] {
        return this[TableNamePropertyMapping[tableName] as keyof this] as TableNameToTableType[T];
    }

    /** Get the URL for the Airtable base. */
    public url(): string {
        return buildUrl(this.baseId);
    }

    /** Fetch a live version of the schema from Airtable's metadata API. */
    public async getSchema(): Promise<BaseSchema> {
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
    public invalidateCache(): void {
        this.formulas.invalidateCache();
        this.primary.invalidateCache();
        this.secondary.invalidateCache();
        this.tertiary.invalidateCache();
    }
}
