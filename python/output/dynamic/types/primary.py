# ==========================================
# Auto-generated file. Do not edit directly.
# ==========================================

# region IMPORTS
from datetime import (
    datetime,
    timedelta,
)
from typing import (
    Any,
    Literal,
    TypedDict,
)
from ...static.special_types import (
    AirtableAttachment,
    AirtableButton,
    AirtableCollaborator,
    RecordId,
)
# endregion


# region OPTIONS
type PrimaryMultipleSelectOption = Literal[
    "Option 1",
    "Option 2",
    "Option 3",
]
"""Select options for `Multiple Select`"""
PrimaryMultipleSelectOptions: list[PrimaryMultipleSelectOption] = [
    "Option 1",
    "Option 2",
    "Option 3",
]
"""Select options for `Multiple Select`"""

PrimaryMultipleSelectOptionNameIdMapping: dict[PrimaryMultipleSelectOption, str] = {
    "Option 1": "selKcXTyNnNHd3nSK",
    "Option 2": "selBUFMeKPuDUvn0a",
    "Option 3": "selmD0luyDWhS6AyO",
}

PrimaryMultipleSelectOptionIdNameMapping: dict[str, PrimaryMultipleSelectOption] = {
    "selKcXTyNnNHd3nSK": "Option 1",
    "selBUFMeKPuDUvn0a": "Option 2",
    "selmD0luyDWhS6AyO": "Option 3",
}

type PrimarySingleSelectOption = Literal[
    "Choice 1",
    "Choice 2",
    "Choice 3",
]
"""Select options for `Single Select`"""
PrimarySingleSelectOptions: list[PrimarySingleSelectOption] = [
    "Choice 1",
    "Choice 2",
    "Choice 3",
]
"""Select options for `Single Select`"""

PrimarySingleSelectOptionNameIdMapping: dict[PrimarySingleSelectOption, str] = {
    "Choice 1": "sellmCNG85TCgc7Pu",
    "Choice 2": "sel4oPckj6NHWKn2u",
    "Choice 3": "selYfbfAsr7X75S2V",
}

PrimarySingleSelectOptionIdNameMapping: dict[str, PrimarySingleSelectOption] = {
    "sellmCNG85TCgc7Pu": "Choice 1",
    "sel4oPckj6NHWKn2u": "Choice 2",
    "selYfbfAsr7X75S2V": "Choice 3",
}

# endregion

# region PRIMARY
type PrimaryField = Literal[
    "Attachment",
    "Auto Number",
    "Button",
    "Checkbox",
    "Created By",
    "Created Time",
    "Currency (float)",
    "Currency (int)",
    "Date",
    "Date (with time)",
    "Duplicate (Name)",
    "Duplicate Name",
    "Duration",
    "Email",
    "Formula (Complex)",
    "Formula (ID)",
    "Formula (Nested)",
    "Formula (Simple)",
    "Last Modified By",
    "Last Modified Time",
    "Link (multiple)",
    "Link (single)",
    "Long Text",
    "Long Text with Rich Text",
    "Lookup",
    "Multiple Select",
    "Number (float)",
    "Number (int)",
    "Percent (float)",
    "Percent (int)",
    "Phone Number",
    "Primary Key",
    "Rating",
    "Rollup",
    "Single Line Text",
    "Single Select",
    "URL",
    "User",
    "User (allow multiple)",
]
"""Field names for `Primary`"""
PrimaryFields: list[PrimaryField] = [
    "Attachment",
    "Auto Number",
    "Button",
    "Checkbox",
    "Created By",
    "Created Time",
    "Currency (float)",
    "Currency (int)",
    "Date",
    "Date (with time)",
    "Duplicate (Name)",
    "Duplicate Name",
    "Duration",
    "Email",
    "Formula (Complex)",
    "Formula (ID)",
    "Formula (Nested)",
    "Formula (Simple)",
    "Last Modified By",
    "Last Modified Time",
    "Link (multiple)",
    "Link (single)",
    "Long Text",
    "Long Text with Rich Text",
    "Lookup",
    "Multiple Select",
    "Number (float)",
    "Number (int)",
    "Percent (float)",
    "Percent (int)",
    "Phone Number",
    "Primary Key",
    "Rating",
    "Rollup",
    "Single Line Text",
    "Single Select",
    "URL",
    "User",
    "User (allow multiple)",
]
"""Field names for `Primary`"""

type PrimaryFieldId = Literal[
    "fldhF2AEuSC1haCZd",
    "fldizvTkxgIn0mC3L",
    "fldY48yKPG16AajtU",
    "fldjQIaAZVegb1FUa",
    "fldGLQhDz2UjjiHG6",
    "fld2YgW382Kt9xltA",
    "fldyh8pzDXiy5abEr",
    "fldBfo74z9hD78hP8",
    "fldC6LfNVvVIxKyQH",
    "fldizYmjpXABGDLTG",
    "fld99XdcwRa5WW6nw",
    "fld95OnbzAUFUTTqZ",
    "fldLTyf6ljS0rhur8",
    "fldHCJoYBiFVsNvP4",
    "fld2vnFc0Bl5IOFUQ",
    "fldcf62YFeIIDHElt",
    "fldXFeHRPBLz6AiWh",
    "fldy1axxaoUToLVC6",
    "fldF8iDttqP0AgzWC",
    "fldMinKh4pa3YX86g",
    "fldFyFheQWczd8oux",
    "fld7F5onkDo6mkmbN",
    "fld8ulc6J0W29M6La",
    "fldHJkxCMC0xo343u",
    "fldbmFmrzYKBktJvE",
    "fld6GTabFmu1xKPvZ",
    "fldmU0X2l4RWd21dd",
    "fldOfPKGmnRPv94QH",
    "fldiGui9ll69N7WOj",
    "fldbAAyWboGulpb4s",
    "fld38tnNpHmoks8C8",
    "fldol5Q4wmQJQvPRy",
    "fldRsmwFwQNZkKLp4",
    "fldGaFgBsDC3IBUdV",
    "fld0BL2lFo9fqcKv3",
    "fldn0GFFtMFpCXUNU",
    "fldLYloz2oP4ymf3B",
    "fldU6SbLp8CSkLcA4",
    "fldBwCDbAVxRj9yg7",
]
"""Field IDs for `Primary`"""
PrimaryFieldIds: list[PrimaryFieldId] = [
    "fldhF2AEuSC1haCZd",
    "fldizvTkxgIn0mC3L",
    "fldY48yKPG16AajtU",
    "fldjQIaAZVegb1FUa",
    "fldGLQhDz2UjjiHG6",
    "fld2YgW382Kt9xltA",
    "fldyh8pzDXiy5abEr",
    "fldBfo74z9hD78hP8",
    "fldC6LfNVvVIxKyQH",
    "fldizYmjpXABGDLTG",
    "fld99XdcwRa5WW6nw",
    "fld95OnbzAUFUTTqZ",
    "fldLTyf6ljS0rhur8",
    "fldHCJoYBiFVsNvP4",
    "fld2vnFc0Bl5IOFUQ",
    "fldcf62YFeIIDHElt",
    "fldXFeHRPBLz6AiWh",
    "fldy1axxaoUToLVC6",
    "fldF8iDttqP0AgzWC",
    "fldMinKh4pa3YX86g",
    "fldFyFheQWczd8oux",
    "fld7F5onkDo6mkmbN",
    "fld8ulc6J0W29M6La",
    "fldHJkxCMC0xo343u",
    "fldbmFmrzYKBktJvE",
    "fld6GTabFmu1xKPvZ",
    "fldmU0X2l4RWd21dd",
    "fldOfPKGmnRPv94QH",
    "fldiGui9ll69N7WOj",
    "fldbAAyWboGulpb4s",
    "fld38tnNpHmoks8C8",
    "fldol5Q4wmQJQvPRy",
    "fldRsmwFwQNZkKLp4",
    "fldGaFgBsDC3IBUdV",
    "fld0BL2lFo9fqcKv3",
    "fldn0GFFtMFpCXUNU",
    "fldLYloz2oP4ymf3B",
    "fldU6SbLp8CSkLcA4",
    "fldBwCDbAVxRj9yg7",
]
"""Field IDs for `Primary`"""

type PrimaryFieldProperty = Literal[
    "attachment",
    "auto_number",
    "button",
    "checkbox",
    "created_by",
    "created_at_time",
    "currency_float",
    "currency_int",
    "date",
    "date_with_time",
    "duplicate_name",
    "duplicate_name_v2",
    "duration",
    "email",
    "formula_complex",
    "formula_id",
    "formula_nested",
    "formula_simple",
    "last_modified_by",
    "last_modified_time",
    "link_multiple",
    "link_single",
    "long_text",
    "long_text_with_rich_text",
    "lookup",
    "multiple_select",
    "number_float",
    "number_int",
    "percent_float",
    "percent_int",
    "phone_number",
    "primary_key",
    "rating",
    "rollup",
    "single_line_text",
    "single_select",
    "url",
    "user",
    "user_allow_multiple",
]
"""Property names for `Primary`"""
PrimaryFieldPropertys: list[PrimaryFieldProperty] = [
    "attachment",
    "auto_number",
    "button",
    "checkbox",
    "created_by",
    "created_at_time",
    "currency_float",
    "currency_int",
    "date",
    "date_with_time",
    "duplicate_name",
    "duplicate_name_v2",
    "duration",
    "email",
    "formula_complex",
    "formula_id",
    "formula_nested",
    "formula_simple",
    "last_modified_by",
    "last_modified_time",
    "link_multiple",
    "link_single",
    "long_text",
    "long_text_with_rich_text",
    "lookup",
    "multiple_select",
    "number_float",
    "number_int",
    "percent_float",
    "percent_int",
    "phone_number",
    "primary_key",
    "rating",
    "rollup",
    "single_line_text",
    "single_select",
    "url",
    "user",
    "user_allow_multiple",
]
"""Property names for `Primary`"""

PrimaryCalculatedFields: list[str] = [
    "Auto Number",
    "Button",
    "Created By",
    "Created Time",
    "Formula (Complex)",
    "Formula (ID)",
    "Formula (Nested)",
    "Formula (Simple)",
    "Last Modified By",
    "Last Modified Time",
    "Lookup",
    "Rollup",
]
"""Calculated fields for `Primary`"""
PrimaryCalculatedFieldIds: list[str] = [
    "fldizvTkxgIn0mC3L",
    "fldY48yKPG16AajtU",
    "fldGLQhDz2UjjiHG6",
    "fld2YgW382Kt9xltA",
    "fld2vnFc0Bl5IOFUQ",
    "fldcf62YFeIIDHElt",
    "fldXFeHRPBLz6AiWh",
    "fldy1axxaoUToLVC6",
    "fldF8iDttqP0AgzWC",
    "fldMinKh4pa3YX86g",
    "fldbmFmrzYKBktJvE",
    "fldGaFgBsDC3IBUdV",
]
"""Calculated fields for `Primary`"""

PrimaryFieldNameIdMapping: dict[PrimaryField, PrimaryFieldId] = {
    "Attachment": "fldhF2AEuSC1haCZd",
    "Auto Number": "fldizvTkxgIn0mC3L",
    "Button": "fldY48yKPG16AajtU",
    "Checkbox": "fldjQIaAZVegb1FUa",
    "Created By": "fldGLQhDz2UjjiHG6",
    "Created Time": "fld2YgW382Kt9xltA",
    "Currency (float)": "fldyh8pzDXiy5abEr",
    "Currency (int)": "fldBfo74z9hD78hP8",
    "Date": "fldC6LfNVvVIxKyQH",
    "Date (with time)": "fldizYmjpXABGDLTG",
    "Duplicate (Name)": "fld99XdcwRa5WW6nw",
    "Duplicate Name": "fld95OnbzAUFUTTqZ",
    "Duration": "fldLTyf6ljS0rhur8",
    "Email": "fldHCJoYBiFVsNvP4",
    "Formula (Complex)": "fld2vnFc0Bl5IOFUQ",
    "Formula (ID)": "fldcf62YFeIIDHElt",
    "Formula (Nested)": "fldXFeHRPBLz6AiWh",
    "Formula (Simple)": "fldy1axxaoUToLVC6",
    "Last Modified By": "fldF8iDttqP0AgzWC",
    "Last Modified Time": "fldMinKh4pa3YX86g",
    "Link (multiple)": "fldFyFheQWczd8oux",
    "Link (single)": "fld7F5onkDo6mkmbN",
    "Long Text": "fld8ulc6J0W29M6La",
    "Long Text with Rich Text": "fldHJkxCMC0xo343u",
    "Lookup": "fldbmFmrzYKBktJvE",
    "Multiple Select": "fld6GTabFmu1xKPvZ",
    "Number (float)": "fldmU0X2l4RWd21dd",
    "Number (int)": "fldOfPKGmnRPv94QH",
    "Percent (float)": "fldiGui9ll69N7WOj",
    "Percent (int)": "fldbAAyWboGulpb4s",
    "Phone Number": "fld38tnNpHmoks8C8",
    "Primary Key": "fldol5Q4wmQJQvPRy",
    "Rating": "fldRsmwFwQNZkKLp4",
    "Rollup": "fldGaFgBsDC3IBUdV",
    "Single Line Text": "fld0BL2lFo9fqcKv3",
    "Single Select": "fldn0GFFtMFpCXUNU",
    "URL": "fldLYloz2oP4ymf3B",
    "User": "fldU6SbLp8CSkLcA4",
    "User (allow multiple)": "fldBwCDbAVxRj9yg7",
}

PrimaryFieldIdNameMapping: dict[PrimaryFieldId, PrimaryField] = {
    "fldhF2AEuSC1haCZd": "Attachment",
    "fldizvTkxgIn0mC3L": "Auto Number",
    "fldY48yKPG16AajtU": "Button",
    "fldjQIaAZVegb1FUa": "Checkbox",
    "fldGLQhDz2UjjiHG6": "Created By",
    "fld2YgW382Kt9xltA": "Created Time",
    "fldyh8pzDXiy5abEr": "Currency (float)",
    "fldBfo74z9hD78hP8": "Currency (int)",
    "fldC6LfNVvVIxKyQH": "Date",
    "fldizYmjpXABGDLTG": "Date (with time)",
    "fld99XdcwRa5WW6nw": "Duplicate (Name)",
    "fld95OnbzAUFUTTqZ": "Duplicate Name",
    "fldLTyf6ljS0rhur8": "Duration",
    "fldHCJoYBiFVsNvP4": "Email",
    "fld2vnFc0Bl5IOFUQ": "Formula (Complex)",
    "fldcf62YFeIIDHElt": "Formula (ID)",
    "fldXFeHRPBLz6AiWh": "Formula (Nested)",
    "fldy1axxaoUToLVC6": "Formula (Simple)",
    "fldF8iDttqP0AgzWC": "Last Modified By",
    "fldMinKh4pa3YX86g": "Last Modified Time",
    "fldFyFheQWczd8oux": "Link (multiple)",
    "fld7F5onkDo6mkmbN": "Link (single)",
    "fld8ulc6J0W29M6La": "Long Text",
    "fldHJkxCMC0xo343u": "Long Text with Rich Text",
    "fldbmFmrzYKBktJvE": "Lookup",
    "fld6GTabFmu1xKPvZ": "Multiple Select",
    "fldmU0X2l4RWd21dd": "Number (float)",
    "fldOfPKGmnRPv94QH": "Number (int)",
    "fldiGui9ll69N7WOj": "Percent (float)",
    "fldbAAyWboGulpb4s": "Percent (int)",
    "fld38tnNpHmoks8C8": "Phone Number",
    "fldol5Q4wmQJQvPRy": "Primary Key",
    "fldRsmwFwQNZkKLp4": "Rating",
    "fldGaFgBsDC3IBUdV": "Rollup",
    "fld0BL2lFo9fqcKv3": "Single Line Text",
    "fldn0GFFtMFpCXUNU": "Single Select",
    "fldLYloz2oP4ymf3B": "URL",
    "fldU6SbLp8CSkLcA4": "User",
    "fldBwCDbAVxRj9yg7": "User (allow multiple)",
}

PrimaryFieldIdPropertyMapping: dict[PrimaryFieldId, PrimaryFieldProperty] = {
    "fldhF2AEuSC1haCZd": "attachment",
    "fldizvTkxgIn0mC3L": "auto_number",
    "fldY48yKPG16AajtU": "button",
    "fldjQIaAZVegb1FUa": "checkbox",
    "fldGLQhDz2UjjiHG6": "created_by",
    "fld2YgW382Kt9xltA": "created_at_time",
    "fldyh8pzDXiy5abEr": "currency_float",
    "fldBfo74z9hD78hP8": "currency_int",
    "fldC6LfNVvVIxKyQH": "date",
    "fldizYmjpXABGDLTG": "date_with_time",
    "fld99XdcwRa5WW6nw": "duplicate_name",
    "fld95OnbzAUFUTTqZ": "duplicate_name_v2",
    "fldLTyf6ljS0rhur8": "duration",
    "fldHCJoYBiFVsNvP4": "email",
    "fld2vnFc0Bl5IOFUQ": "formula_complex",
    "fldcf62YFeIIDHElt": "formula_id",
    "fldXFeHRPBLz6AiWh": "formula_nested",
    "fldy1axxaoUToLVC6": "formula_simple",
    "fldF8iDttqP0AgzWC": "last_modified_by",
    "fldMinKh4pa3YX86g": "last_modified_time",
    "fldFyFheQWczd8oux": "link_multiple",
    "fld7F5onkDo6mkmbN": "link_single",
    "fld8ulc6J0W29M6La": "long_text",
    "fldHJkxCMC0xo343u": "long_text_with_rich_text",
    "fldbmFmrzYKBktJvE": "lookup",
    "fld6GTabFmu1xKPvZ": "multiple_select",
    "fldmU0X2l4RWd21dd": "number_float",
    "fldOfPKGmnRPv94QH": "number_int",
    "fldiGui9ll69N7WOj": "percent_float",
    "fldbAAyWboGulpb4s": "percent_int",
    "fld38tnNpHmoks8C8": "phone_number",
    "fldol5Q4wmQJQvPRy": "primary_key",
    "fldRsmwFwQNZkKLp4": "rating",
    "fldGaFgBsDC3IBUdV": "rollup",
    "fld0BL2lFo9fqcKv3": "single_line_text",
    "fldn0GFFtMFpCXUNU": "single_select",
    "fldLYloz2oP4ymf3B": "url",
    "fldU6SbLp8CSkLcA4": "user",
    "fldBwCDbAVxRj9yg7": "user_allow_multiple",
}

PrimaryFieldPropertyIdMapping: dict[PrimaryFieldProperty, PrimaryFieldId] = {
    "attachment": "fldhF2AEuSC1haCZd",
    "auto_number": "fldizvTkxgIn0mC3L",
    "button": "fldY48yKPG16AajtU",
    "checkbox": "fldjQIaAZVegb1FUa",
    "created_by": "fldGLQhDz2UjjiHG6",
    "created_at_time": "fld2YgW382Kt9xltA",
    "currency_float": "fldyh8pzDXiy5abEr",
    "currency_int": "fldBfo74z9hD78hP8",
    "date": "fldC6LfNVvVIxKyQH",
    "date_with_time": "fldizYmjpXABGDLTG",
    "duplicate_name": "fld99XdcwRa5WW6nw",
    "duplicate_name_v2": "fld95OnbzAUFUTTqZ",
    "duration": "fldLTyf6ljS0rhur8",
    "email": "fldHCJoYBiFVsNvP4",
    "formula_complex": "fld2vnFc0Bl5IOFUQ",
    "formula_id": "fldcf62YFeIIDHElt",
    "formula_nested": "fldXFeHRPBLz6AiWh",
    "formula_simple": "fldy1axxaoUToLVC6",
    "last_modified_by": "fldF8iDttqP0AgzWC",
    "last_modified_time": "fldMinKh4pa3YX86g",
    "link_multiple": "fldFyFheQWczd8oux",
    "link_single": "fld7F5onkDo6mkmbN",
    "long_text": "fld8ulc6J0W29M6La",
    "long_text_with_rich_text": "fldHJkxCMC0xo343u",
    "lookup": "fldbmFmrzYKBktJvE",
    "multiple_select": "fld6GTabFmu1xKPvZ",
    "number_float": "fldmU0X2l4RWd21dd",
    "number_int": "fldOfPKGmnRPv94QH",
    "percent_float": "fldiGui9ll69N7WOj",
    "percent_int": "fldbAAyWboGulpb4s",
    "phone_number": "fld38tnNpHmoks8C8",
    "primary_key": "fldol5Q4wmQJQvPRy",
    "rating": "fldRsmwFwQNZkKLp4",
    "rollup": "fldGaFgBsDC3IBUdV",
    "single_line_text": "fld0BL2lFo9fqcKv3",
    "single_select": "fldn0GFFtMFpCXUNU",
    "url": "fldLYloz2oP4ymf3B",
    "user": "fldU6SbLp8CSkLcA4",
    "user_allow_multiple": "fldBwCDbAVxRj9yg7",
}

PrimaryFieldNamePropertyMapping: dict[PrimaryField, PrimaryFieldProperty] = {
    "Attachment": "attachment",
    "Auto Number": "auto_number",
    "Button": "button",
    "Checkbox": "checkbox",
    "Created By": "created_by",
    "Created Time": "created_at_time",
    "Currency (float)": "currency_float",
    "Currency (int)": "currency_int",
    "Date": "date",
    "Date (with time)": "date_with_time",
    "Duplicate (Name)": "duplicate_name",
    "Duplicate Name": "duplicate_name_v2",
    "Duration": "duration",
    "Email": "email",
    "Formula (Complex)": "formula_complex",
    "Formula (ID)": "formula_id",
    "Formula (Nested)": "formula_nested",
    "Formula (Simple)": "formula_simple",
    "Last Modified By": "last_modified_by",
    "Last Modified Time": "last_modified_time",
    "Link (multiple)": "link_multiple",
    "Link (single)": "link_single",
    "Long Text": "long_text",
    "Long Text with Rich Text": "long_text_with_rich_text",
    "Lookup": "lookup",
    "Multiple Select": "multiple_select",
    "Number (float)": "number_float",
    "Number (int)": "number_int",
    "Percent (float)": "percent_float",
    "Percent (int)": "percent_int",
    "Phone Number": "phone_number",
    "Primary Key": "primary_key",
    "Rating": "rating",
    "Rollup": "rollup",
    "Single Line Text": "single_line_text",
    "Single Select": "single_select",
    "URL": "url",
    "User": "user",
    "User (allow multiple)": "user_allow_multiple",
}

PrimaryFieldPropertyNameMapping: dict[PrimaryFieldProperty, PrimaryField] = {
    "attachment": "Attachment",
    "auto_number": "Auto Number",
    "button": "Button",
    "checkbox": "Checkbox",
    "created_by": "Created By",
    "created_at_time": "Created Time",
    "currency_float": "Currency (float)",
    "currency_int": "Currency (int)",
    "date": "Date",
    "date_with_time": "Date (with time)",
    "duplicate_name": "Duplicate (Name)",
    "duplicate_name_v2": "Duplicate Name",
    "duration": "Duration",
    "email": "Email",
    "formula_complex": "Formula (Complex)",
    "formula_id": "Formula (ID)",
    "formula_nested": "Formula (Nested)",
    "formula_simple": "Formula (Simple)",
    "last_modified_by": "Last Modified By",
    "last_modified_time": "Last Modified Time",
    "link_multiple": "Link (multiple)",
    "link_single": "Link (single)",
    "long_text": "Long Text",
    "long_text_with_rich_text": "Long Text with Rich Text",
    "lookup": "Lookup",
    "multiple_select": "Multiple Select",
    "number_float": "Number (float)",
    "number_int": "Number (int)",
    "percent_float": "Percent (float)",
    "percent_int": "Percent (int)",
    "phone_number": "Phone Number",
    "primary_key": "Primary Key",
    "rating": "Rating",
    "rollup": "Rollup",
    "single_line_text": "Single Line Text",
    "single_select": "Single Select",
    "url": "URL",
    "user": "User",
    "user_allow_multiple": "User (allow multiple)",
}

class PrimaryFieldsDict(TypedDict, total=False):
    fldhF2AEuSC1haCZd: list[AirtableAttachment]
    fldizvTkxgIn0mC3L: int
    fldY48yKPG16AajtU: AirtableButton
    fldjQIaAZVegb1FUa: bool
    fldGLQhDz2UjjiHG6: AirtableCollaborator
    fld2YgW382Kt9xltA: datetime
    fldyh8pzDXiy5abEr: float
    fldBfo74z9hD78hP8: float
    fldC6LfNVvVIxKyQH: datetime
    fldizYmjpXABGDLTG: datetime
    fld99XdcwRa5WW6nw: str
    fld95OnbzAUFUTTqZ: str
    fldLTyf6ljS0rhur8: timedelta
    fldHCJoYBiFVsNvP4: str
    fld2vnFc0Bl5IOFUQ: str
    fldcf62YFeIIDHElt: str
    fldXFeHRPBLz6AiWh: str
    fldy1axxaoUToLVC6: float
    fldF8iDttqP0AgzWC: AirtableCollaborator
    fldMinKh4pa3YX86g: datetime
    fldFyFheQWczd8oux: list[RecordId]
    fld7F5onkDo6mkmbN: RecordId
    fld8ulc6J0W29M6La: str
    fldHJkxCMC0xo343u: str
    fldbmFmrzYKBktJvE: list[str]
    fld6GTabFmu1xKPvZ: list[PrimaryMultipleSelectOption]
    fldmU0X2l4RWd21dd: float
    fldOfPKGmnRPv94QH: float
    fldiGui9ll69N7WOj: float
    fldbAAyWboGulpb4s: float
    fld38tnNpHmoks8C8: str
    fldol5Q4wmQJQvPRy: str
    fldRsmwFwQNZkKLp4: Any
    fldGaFgBsDC3IBUdV: list[str]
    fld0BL2lFo9fqcKv3: str
    fldn0GFFtMFpCXUNU: PrimarySingleSelectOption
    fldLYloz2oP4ymf3B: str
    fldU6SbLp8CSkLcA4: AirtableCollaborator
    fldBwCDbAVxRj9yg7: list[AirtableCollaborator]


type PrimaryView = Literal[
    "Grid view",
    "Filter by View",
]
"""View names for `Primary`"""
PrimaryViews: list[PrimaryView] = [
    "Grid view",
    "Filter by View",
]
"""View names for `Primary`"""

type PrimaryViewId = Literal[
    "viwvPRDMaHyldUpmd",
    "viwHlcwGu4xthX1gf",
]
"""View IDs for `Primary`"""
PrimaryViewIds: list[PrimaryViewId] = [
    "viwvPRDMaHyldUpmd",
    "viwHlcwGu4xthX1gf",
]
"""View IDs for `Primary`"""

PrimaryViewNameIdMapping: dict[PrimaryView, PrimaryViewId] = {
    "Grid view": "viwvPRDMaHyldUpmd",
    "Filter by View": "viwHlcwGu4xthX1gf",
}

PrimaryViewIdNameMapping: dict[PrimaryViewId, PrimaryView] = {
    "viwvPRDMaHyldUpmd": "Grid view",
    "viwHlcwGu4xthX1gf": "Filter by View",
}

# endregion

