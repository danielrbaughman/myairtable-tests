# ==========================================
# Auto-generated file. Do not edit directly.
# ==========================================

# region IMPORTS
from datetime import datetime, timedelta
from typing import Any, Literal, TypedDict

from ...static.special_types import AirtableAttachment, AirtableButton, AirtableCollaborator, RecordId
# endregion


# region OPTIONS
# endregion

# region TERTIARY
type TertiaryField = Literal[
    "Name",
    "Secondary",
    "Value",
]
"""Field names for `Tertiary`"""
TertiaryFields: list[TertiaryField] = [
    "Name",
    "Secondary",
    "Value",
]
"""Field names for `Tertiary`"""

type TertiaryFieldId = Literal[
    "fldwzqKxsRnPZJ2Ll",
    "fld8lCuUXpEXkIeYv",
    "fldjNLBh2UccM64h5",
]
"""Field IDs for `Tertiary`"""
TertiaryFieldIds: list[TertiaryFieldId] = [
    "fldwzqKxsRnPZJ2Ll",
    "fld8lCuUXpEXkIeYv",
    "fldjNLBh2UccM64h5",
]
"""Field IDs for `Tertiary`"""

type TertiaryFieldProperty = Literal[
    "name",
    "secondary",
    "value",
]
"""Property names for `Tertiary`"""
TertiaryFieldPropertys: list[TertiaryFieldProperty] = [
    "name",
    "secondary",
    "value",
]
"""Property names for `Tertiary`"""

TertiaryCalculatedFields: list[str] = [
]
"""Calculated fields for `Tertiary`"""
TertiaryCalculatedFieldIds: list[str] = [
]
"""Calculated fields for `Tertiary`"""

TertiaryFieldNameIdMapping: dict[TertiaryField, TertiaryFieldId] = {
    "Name": "fldwzqKxsRnPZJ2Ll",
    "Secondary": "fld8lCuUXpEXkIeYv",
    "Value": "fldjNLBh2UccM64h5",
}

TertiaryFieldIdNameMapping: dict[TertiaryFieldId, TertiaryField] = {
    "fldwzqKxsRnPZJ2Ll": "Name",
    "fld8lCuUXpEXkIeYv": "Secondary",
    "fldjNLBh2UccM64h5": "Value",
}

TertiaryFieldIdPropertyMapping: dict[TertiaryFieldId, TertiaryFieldProperty] = {
    "fldwzqKxsRnPZJ2Ll": "name",
    "fld8lCuUXpEXkIeYv": "secondary",
    "fldjNLBh2UccM64h5": "value",
}

TertiaryFieldPropertyIdMapping: dict[TertiaryFieldProperty, TertiaryFieldId] = {
    "name": "fldwzqKxsRnPZJ2Ll",
    "secondary": "fld8lCuUXpEXkIeYv",
    "value": "fldjNLBh2UccM64h5",
}

TertiaryFieldNamePropertyMapping: dict[TertiaryField, TertiaryFieldProperty] = {
    "Name": "name",
    "Secondary": "secondary",
    "Value": "value",
}

TertiaryFieldPropertyNameMapping: dict[TertiaryFieldProperty, TertiaryField] = {
    "name": "Name",
    "secondary": "Secondary",
    "value": "Value",
}

class TertiaryFieldsDict(TypedDict, total=False):
    fldwzqKxsRnPZJ2Ll: str
    fld8lCuUXpEXkIeYv: list[RecordId]
    fldjNLBh2UccM64h5: str


type TertiaryView = Literal[
    "Grid view",
]
"""View names for `Tertiary`"""
TertiaryViews: list[TertiaryView] = [
    "Grid view",
]
"""View names for `Tertiary`"""

type TertiaryViewId = Literal[
    "viwdp3tOB8ooOCvP4",
]
"""View IDs for `Tertiary`"""
TertiaryViewIds: list[TertiaryViewId] = [
    "viwdp3tOB8ooOCvP4",
]
"""View IDs for `Tertiary`"""

TertiaryViewNameIdMapping: dict[TertiaryView, TertiaryViewId] = {
    "Grid view": "viwdp3tOB8ooOCvP4",
}

TertiaryViewIdNameMapping: dict[TertiaryViewId, TertiaryView] = {
    "viwdp3tOB8ooOCvP4": "Grid view",
}

# endregion

