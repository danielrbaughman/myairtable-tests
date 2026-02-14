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

# region SECONDARY
SecondaryField = Literal[
    "Link to Tertiary",
    "Name",
    "Primary",
    "Primary 2",
    "Value",
]
"""Field names for `Secondary`"""
SecondaryFields: list[SecondaryField] = [
    "Link to Tertiary",
    "Name",
    "Primary",
    "Primary 2",
    "Value",
]
"""Field names for `Secondary`"""

SecondaryFieldId = Literal[
    "fldKR6tdbnOBRCtdQ",
    "fld1RagdJ09mpWhzM",
    "fldl0nB9WRFSdqlii",
    "fldgoE2oZmXmKkQca",
    "fldi6Mxh5H1gPGxFX",
]
"""Field IDs for `Secondary`"""
SecondaryFieldIds: list[SecondaryFieldId] = [
    "fldKR6tdbnOBRCtdQ",
    "fld1RagdJ09mpWhzM",
    "fldl0nB9WRFSdqlii",
    "fldgoE2oZmXmKkQca",
    "fldi6Mxh5H1gPGxFX",
]
"""Field IDs for `Secondary`"""

SecondaryFieldProperty = Literal[
    "link_to_tertiary",
    "name",
    "primary",
    "primary_2",
    "value",
]
"""Property names for `Secondary`"""
SecondaryFieldPropertys: list[SecondaryFieldProperty] = [
    "link_to_tertiary",
    "name",
    "primary",
    "primary_2",
    "value",
]
"""Property names for `Secondary`"""

SecondaryCalculatedFields: list[str] = [
]
"""Calculated fields for `Secondary`"""
SecondaryCalculatedFieldIds: list[str] = [
]
"""Calculated fields for `Secondary`"""

SecondaryFieldNameIdMapping: dict[SecondaryField, SecondaryFieldId] = {
    "Link to Tertiary": "fldKR6tdbnOBRCtdQ",
    "Name": "fld1RagdJ09mpWhzM",
    "Primary": "fldl0nB9WRFSdqlii",
    "Primary 2": "fldgoE2oZmXmKkQca",
    "Value": "fldi6Mxh5H1gPGxFX",
}

SecondaryFieldIdNameMapping: dict[SecondaryFieldId, SecondaryField] = {
    "fldKR6tdbnOBRCtdQ": "Link to Tertiary",
    "fld1RagdJ09mpWhzM": "Name",
    "fldl0nB9WRFSdqlii": "Primary",
    "fldgoE2oZmXmKkQca": "Primary 2",
    "fldi6Mxh5H1gPGxFX": "Value",
}

SecondaryFieldIdPropertyMapping: dict[SecondaryFieldId, SecondaryFieldProperty] = {
    "fldKR6tdbnOBRCtdQ": "link_to_tertiary",
    "fld1RagdJ09mpWhzM": "name",
    "fldl0nB9WRFSdqlii": "primary",
    "fldgoE2oZmXmKkQca": "primary_2",
    "fldi6Mxh5H1gPGxFX": "value",
}

SecondaryFieldPropertyIdMapping: dict[SecondaryFieldProperty, SecondaryFieldId] = {
    "link_to_tertiary": "fldKR6tdbnOBRCtdQ",
    "name": "fld1RagdJ09mpWhzM",
    "primary": "fldl0nB9WRFSdqlii",
    "primary_2": "fldgoE2oZmXmKkQca",
    "value": "fldi6Mxh5H1gPGxFX",
}

SecondaryFieldNamePropertyMapping: dict[SecondaryField, SecondaryFieldProperty] = {
    "Link to Tertiary": "link_to_tertiary",
    "Name": "name",
    "Primary": "primary",
    "Primary 2": "primary_2",
    "Value": "value",
}

SecondaryFieldPropertyNameMapping: dict[SecondaryFieldProperty, SecondaryField] = {
    "link_to_tertiary": "Link to Tertiary",
    "name": "Name",
    "primary": "Primary",
    "primary_2": "Primary 2",
    "value": "Value",
}

class SecondaryFieldsDict(TypedDict, total=False):
    fldKR6tdbnOBRCtdQ: list[RecordId]
    fld1RagdJ09mpWhzM: str
    fldl0nB9WRFSdqlii: list[RecordId]
    fldgoE2oZmXmKkQca: list[RecordId]
    fldi6Mxh5H1gPGxFX: str


SecondaryView = Literal[
    "Grid view",
]
"""View names for `Secondary`"""
SecondaryViews: list[SecondaryView] = [
    "Grid view",
]
"""View names for `Secondary`"""

SecondaryViewId = Literal[
    "viwTml4ZHkNi8kJbD",
]
"""View IDs for `Secondary`"""
SecondaryViewIds: list[SecondaryViewId] = [
    "viwTml4ZHkNi8kJbD",
]
"""View IDs for `Secondary`"""

SecondaryViewNameIdMapping: dict[SecondaryView, SecondaryViewId] = {
    "Grid view": "viwTml4ZHkNi8kJbD",
}

SecondaryViewIdNameMapping: dict[SecondaryViewId, SecondaryView] = {
    "viwTml4ZHkNi8kJbD": "Grid view",
}

# endregion

