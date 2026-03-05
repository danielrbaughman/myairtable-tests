# ==========================================
# Auto-generated file. Do not edit directly.
# ==========================================

from typing import Literal
from .formulas import FormulasField, FormulasFields, FormulasFieldNameIdMapping
from .primary import PrimaryField, PrimaryFields, PrimaryFieldNameIdMapping
from .secondary import SecondaryField, SecondaryFields, SecondaryFieldNameIdMapping
from .tertiary import TertiaryField, TertiaryFields, TertiaryFieldNameIdMapping

type TableName = Literal[
    "Formulas",
    "Primary",
    "Secondary",
    "Tertiary",
]
TableNames: list[TableName] = [
    "Formulas",
    "Primary",
    "Secondary",
    "Tertiary",
]

type TableId = Literal[
    "tblnuYBsMdXNDsuRc",
    "tblmb3iqgpNS1ysV2",
    "tblPPScS3XMuFkDYN",
    "tblLFoLxEdWlxjmLP",
]
TableIds: list[TableId] = [
    "tblnuYBsMdXNDsuRc",
    "tblmb3iqgpNS1ysV2",
    "tblPPScS3XMuFkDYN",
    "tblLFoLxEdWlxjmLP",
]

TableNameIdMapping: dict[TableName, TableId] = {
    "Formulas": "tblnuYBsMdXNDsuRc",
    "Primary": "tblmb3iqgpNS1ysV2",
    "Secondary": "tblPPScS3XMuFkDYN",
    "Tertiary": "tblLFoLxEdWlxjmLP",
}

TableIdNameMapping: dict[TableId, TableName] = {
    "tblnuYBsMdXNDsuRc": "Formulas",
    "tblmb3iqgpNS1ysV2": "Primary",
    "tblPPScS3XMuFkDYN": "Secondary",
    "tblLFoLxEdWlxjmLP": "Tertiary",
}

TableNamePropertyMapping: dict[TableName, str] = {
    "Formulas": "formulas",
    "Primary": "primary",
    "Secondary": "secondary",
    "Tertiary": "tertiary",
}

TableIdToFieldNameIdMapping: dict[TableId, dict[str, str]] = {
    "tblnuYBsMdXNDsuRc": FormulasFieldNameIdMapping,
    "tblmb3iqgpNS1ysV2": PrimaryFieldNameIdMapping,
    "tblPPScS3XMuFkDYN": SecondaryFieldNameIdMapping,
    "tblLFoLxEdWlxjmLP": TertiaryFieldNameIdMapping,
}

TableIdToFieldNamesTypeMapping: dict[TableId, str] = {
    "tblnuYBsMdXNDsuRc": FormulasField,
    "tblmb3iqgpNS1ysV2": PrimaryField,
    "tblPPScS3XMuFkDYN": SecondaryField,
    "tblLFoLxEdWlxjmLP": TertiaryField,
}

TableIdToFieldNamesListMapping: dict[TableId, list[str]] = {
    "tblnuYBsMdXNDsuRc": FormulasFields,
    "tblmb3iqgpNS1ysV2": PrimaryFields,
    "tblPPScS3XMuFkDYN": SecondaryFields,
    "tblLFoLxEdWlxjmLP": TertiaryFields,
}

TableIdToFieldNameToFieldIdMapping: dict[TableId, dict[str, str]] = {
    "tblnuYBsMdXNDsuRc": FormulasFieldNameIdMapping,
    "tblmb3iqgpNS1ysV2": PrimaryFieldNameIdMapping,
    "tblPPScS3XMuFkDYN": SecondaryFieldNameIdMapping,
    "tblLFoLxEdWlxjmLP": TertiaryFieldNameIdMapping,
}

