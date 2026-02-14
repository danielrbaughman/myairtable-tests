// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

import { ExtendedAirtableOptions } from "../static/special-types";
import { getApiKey, getBaseId, setAirtableConfig } from "../static/helpers";
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
    /** `Formulas` (tblnuYBsMdXNDsuRc) */
    public formulas: FormulasTable;
    /** `Primary` (tblmb3iqgpNS1ysV2) */
    public primary: PrimaryTable;
    /** `Secondary` (tblPPScS3XMuFkDYN) */
    public secondary: SecondaryTable;
    /** `Tertiary` (tblLFoLxEdWlxjmLP) */
    public tertiary: TertiaryTable;

    constructor(options?: ExtendedAirtableOptions) {
        const _baseId = options?.baseId || getBaseId();
        const _options = {
              apiKey: options?.apiKey ?? getApiKey(),
              apiVersion: options?.apiVersion,
              customHeaders: options?.customHeaders,
              endpointUrl: options?.endpointUrl,
              noRetryIfRateLimited: options?.noRetryIfRateLimited ?? false,
              requestTimeout: options?.requestTimeout,
        };
        setAirtableConfig(_baseId, _options);
        this.formulas = new FormulasTable(_baseId, _options);
        this.primary = new PrimaryTable(_baseId, _options);
        this.secondary = new SecondaryTable(_baseId, _options);
        this.tertiary = new TertiaryTable(_baseId, _options);
    }


    /** Get a table by its Airtable name. */
    public table<T extends TableName>(tableName: T): TableNameToTableType[T] {
        return this[TableNamePropertyMapping[tableName] as keyof this] as TableNameToTableType[T];
    }
}
