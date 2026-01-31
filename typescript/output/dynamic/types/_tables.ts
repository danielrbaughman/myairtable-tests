// ==========================================
// Auto-generated file. Do not edit directly.
// ==========================================

// #region IMPORTS
import { PrimaryFieldNameIdMapping } from './primary';
import { SecondaryFieldNameIdMapping } from './secondary';
// #endregion


// #region TABLES
export type TableName = 
    "Primary" |
    "Secondary"
export const TableNames: TableName[] = [
    "Primary",
    "Secondary",
]

export type TablePropertyName = 
    "primary" |
    "secondary"
export const TablePropertyNames: TablePropertyName[] = [
    "primary",
    "secondary",
]

export type TableId = 
    "tblmb3iqgpNS1ysV2" |
    "tblPPScS3XMuFkDYN"
export const TableIds: TableId[] = [
    "tblmb3iqgpNS1ysV2",
    "tblPPScS3XMuFkDYN",
]

export const TableNameIdMapping: Record<TableName, TableId> = {
    "Primary": "tblmb3iqgpNS1ysV2",
    "Secondary": "tblPPScS3XMuFkDYN",
}

export const TableIdNameMapping: Record<TableId, TableName> = {
    "tblmb3iqgpNS1ysV2": "Primary",
    "tblPPScS3XMuFkDYN": "Secondary",
}

export const TableNamePropertyMapping: Record<TableName, string> = {
    "Primary": "primary",
    "Secondary": "secondary",
}

export const TableIdToFieldNameIdMapping: Record<TableId, Record<string, string>> = {
    "tblmb3iqgpNS1ysV2": PrimaryFieldNameIdMapping,
    "tblPPScS3XMuFkDYN": SecondaryFieldNameIdMapping,
}

// #endregion

