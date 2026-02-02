// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

// #region IMPORTS
import { PrimaryFieldNameIdMapping } from './primary';
import { SecondaryFieldNameIdMapping } from './secondary';
import { TertiaryFieldNameIdMapping } from './tertiary';
// #endregion


// #region TABLES
export type TableName = 
    "Primary" |
    "Secondary" |
    "Tertiary"
export const TableNames: TableName[] = [
    "Primary",
    "Secondary",
    "Tertiary",
]

export type TablePropertyName = 
    "primary" |
    "secondary" |
    "tertiary"
export const TablePropertyNames: TablePropertyName[] = [
    "primary",
    "secondary",
    "tertiary",
]

export type TableId = 
    "tblmb3iqgpNS1ysV2" |
    "tblPPScS3XMuFkDYN" |
    "tblLFoLxEdWlxjmLP"
export const TableIds: TableId[] = [
    "tblmb3iqgpNS1ysV2",
    "tblPPScS3XMuFkDYN",
    "tblLFoLxEdWlxjmLP",
]

export const TableNameIdMapping: Record<TableName, TableId> = {
    "Primary": "tblmb3iqgpNS1ysV2",
    "Secondary": "tblPPScS3XMuFkDYN",
    "Tertiary": "tblLFoLxEdWlxjmLP",
}

export const TableIdNameMapping: Record<TableId, TableName> = {
    "tblmb3iqgpNS1ysV2": "Primary",
    "tblPPScS3XMuFkDYN": "Secondary",
    "tblLFoLxEdWlxjmLP": "Tertiary",
}

export const TableNamePropertyMapping: Record<TableName, string> = {
    "Primary": "primary",
    "Secondary": "secondary",
    "Tertiary": "tertiary",
}

export const TableIdToFieldNameIdMapping: Record<TableId, Record<string, string>> = {
    "tblmb3iqgpNS1ysV2": PrimaryFieldNameIdMapping,
    "tblPPScS3XMuFkDYN": SecondaryFieldNameIdMapping,
    "tblLFoLxEdWlxjmLP": TertiaryFieldNameIdMapping,
}

// #endregion

