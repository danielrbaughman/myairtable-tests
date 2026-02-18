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

# region FORMULAS
type FormulasField = Literal[
    "Date Formula",
    "First Date",
    "First Number",
    "First Text",
    "Math Formula",
    "Primary Key",
    "Second Date",
    "Second Number",
    "Second Text",
    "Text Formula",
    "Third Date",
    "Third Number",
    "Third Text",
]
"""Field names for `Formulas`"""
FormulasFields: list[FormulasField] = [
    "Date Formula",
    "First Date",
    "First Number",
    "First Text",
    "Math Formula",
    "Primary Key",
    "Second Date",
    "Second Number",
    "Second Text",
    "Text Formula",
    "Third Date",
    "Third Number",
    "Third Text",
]
"""Field names for `Formulas`"""

type FormulasFieldId = Literal[
    "fldY7kjaklLeoSgGd",
    "fldlZT521Iy0FFXFL",
    "fldA04pqfjMkGXcZU",
    "fldHJuw6pAujnHvkP",
    "fldlSuvoeWGokSz8Z",
    "fldLZFrZKvSCS4dKb",
    "fld1LZ3Ebpt0LaMmu",
    "fldj5nAkal5y8OOZg",
    "fldA2boNwwsiuvXw1",
    "flddvzeqt7FJpQ9NX",
    "fldxSQRRn8W879aiU",
    "fld5NBdekrAUzu4Fi",
    "fldfruPf8V9K6qIAN",
]
"""Field IDs for `Formulas`"""
FormulasFieldIds: list[FormulasFieldId] = [
    "fldY7kjaklLeoSgGd",
    "fldlZT521Iy0FFXFL",
    "fldA04pqfjMkGXcZU",
    "fldHJuw6pAujnHvkP",
    "fldlSuvoeWGokSz8Z",
    "fldLZFrZKvSCS4dKb",
    "fld1LZ3Ebpt0LaMmu",
    "fldj5nAkal5y8OOZg",
    "fldA2boNwwsiuvXw1",
    "flddvzeqt7FJpQ9NX",
    "fldxSQRRn8W879aiU",
    "fld5NBdekrAUzu4Fi",
    "fldfruPf8V9K6qIAN",
]
"""Field IDs for `Formulas`"""

type FormulasFieldProperty = Literal[
    "date_formula",
    "first_date",
    "first_number",
    "first_text",
    "math_formula",
    "primary_key",
    "second_date",
    "second_number",
    "second_text",
    "text_formula",
    "third_date",
    "third_number",
    "third_text",
]
"""Property names for `Formulas`"""
FormulasFieldPropertys: list[FormulasFieldProperty] = [
    "date_formula",
    "first_date",
    "first_number",
    "first_text",
    "math_formula",
    "primary_key",
    "second_date",
    "second_number",
    "second_text",
    "text_formula",
    "third_date",
    "third_number",
    "third_text",
]
"""Property names for `Formulas`"""

FormulasCalculatedFields: list[str] = [
    "Date Formula",
    "Math Formula",
    "Text Formula",
]
"""Calculated fields for `Formulas`"""
FormulasCalculatedFieldIds: list[str] = [
    "fldY7kjaklLeoSgGd",
    "fldlSuvoeWGokSz8Z",
    "flddvzeqt7FJpQ9NX",
]
"""Calculated fields for `Formulas`"""

FormulasFieldNameIdMapping: dict[FormulasField, FormulasFieldId] = {
    "Date Formula": "fldY7kjaklLeoSgGd",
    "First Date": "fldlZT521Iy0FFXFL",
    "First Number": "fldA04pqfjMkGXcZU",
    "First Text": "fldHJuw6pAujnHvkP",
    "Math Formula": "fldlSuvoeWGokSz8Z",
    "Primary Key": "fldLZFrZKvSCS4dKb",
    "Second Date": "fld1LZ3Ebpt0LaMmu",
    "Second Number": "fldj5nAkal5y8OOZg",
    "Second Text": "fldA2boNwwsiuvXw1",
    "Text Formula": "flddvzeqt7FJpQ9NX",
    "Third Date": "fldxSQRRn8W879aiU",
    "Third Number": "fld5NBdekrAUzu4Fi",
    "Third Text": "fldfruPf8V9K6qIAN",
}

FormulasFieldIdNameMapping: dict[FormulasFieldId, FormulasField] = {
    "fldY7kjaklLeoSgGd": "Date Formula",
    "fldlZT521Iy0FFXFL": "First Date",
    "fldA04pqfjMkGXcZU": "First Number",
    "fldHJuw6pAujnHvkP": "First Text",
    "fldlSuvoeWGokSz8Z": "Math Formula",
    "fldLZFrZKvSCS4dKb": "Primary Key",
    "fld1LZ3Ebpt0LaMmu": "Second Date",
    "fldj5nAkal5y8OOZg": "Second Number",
    "fldA2boNwwsiuvXw1": "Second Text",
    "flddvzeqt7FJpQ9NX": "Text Formula",
    "fldxSQRRn8W879aiU": "Third Date",
    "fld5NBdekrAUzu4Fi": "Third Number",
    "fldfruPf8V9K6qIAN": "Third Text",
}

FormulasFieldIdPropertyMapping: dict[FormulasFieldId, FormulasFieldProperty] = {
    "fldY7kjaklLeoSgGd": "date_formula",
    "fldlZT521Iy0FFXFL": "first_date",
    "fldA04pqfjMkGXcZU": "first_number",
    "fldHJuw6pAujnHvkP": "first_text",
    "fldlSuvoeWGokSz8Z": "math_formula",
    "fldLZFrZKvSCS4dKb": "primary_key",
    "fld1LZ3Ebpt0LaMmu": "second_date",
    "fldj5nAkal5y8OOZg": "second_number",
    "fldA2boNwwsiuvXw1": "second_text",
    "flddvzeqt7FJpQ9NX": "text_formula",
    "fldxSQRRn8W879aiU": "third_date",
    "fld5NBdekrAUzu4Fi": "third_number",
    "fldfruPf8V9K6qIAN": "third_text",
}

FormulasFieldPropertyIdMapping: dict[FormulasFieldProperty, FormulasFieldId] = {
    "date_formula": "fldY7kjaklLeoSgGd",
    "first_date": "fldlZT521Iy0FFXFL",
    "first_number": "fldA04pqfjMkGXcZU",
    "first_text": "fldHJuw6pAujnHvkP",
    "math_formula": "fldlSuvoeWGokSz8Z",
    "primary_key": "fldLZFrZKvSCS4dKb",
    "second_date": "fld1LZ3Ebpt0LaMmu",
    "second_number": "fldj5nAkal5y8OOZg",
    "second_text": "fldA2boNwwsiuvXw1",
    "text_formula": "flddvzeqt7FJpQ9NX",
    "third_date": "fldxSQRRn8W879aiU",
    "third_number": "fld5NBdekrAUzu4Fi",
    "third_text": "fldfruPf8V9K6qIAN",
}

FormulasFieldNamePropertyMapping: dict[FormulasField, FormulasFieldProperty] = {
    "Date Formula": "date_formula",
    "First Date": "first_date",
    "First Number": "first_number",
    "First Text": "first_text",
    "Math Formula": "math_formula",
    "Primary Key": "primary_key",
    "Second Date": "second_date",
    "Second Number": "second_number",
    "Second Text": "second_text",
    "Text Formula": "text_formula",
    "Third Date": "third_date",
    "Third Number": "third_number",
    "Third Text": "third_text",
}

FormulasFieldPropertyNameMapping: dict[FormulasFieldProperty, FormulasField] = {
    "date_formula": "Date Formula",
    "first_date": "First Date",
    "first_number": "First Number",
    "first_text": "First Text",
    "math_formula": "Math Formula",
    "primary_key": "Primary Key",
    "second_date": "Second Date",
    "second_number": "Second Number",
    "second_text": "Second Text",
    "text_formula": "Text Formula",
    "third_date": "Third Date",
    "third_number": "Third Number",
    "third_text": "Third Text",
}

class FormulasFieldsDict(TypedDict, total=False):
    fldY7kjaklLeoSgGd: str
    fldlZT521Iy0FFXFL: datetime
    fldA04pqfjMkGXcZU: float
    fldHJuw6pAujnHvkP: str
    fldlSuvoeWGokSz8Z: str
    fldLZFrZKvSCS4dKb: str
    fld1LZ3Ebpt0LaMmu: datetime
    fldj5nAkal5y8OOZg: float
    fldA2boNwwsiuvXw1: str
    flddvzeqt7FJpQ9NX: str
    fldxSQRRn8W879aiU: datetime
    fld5NBdekrAUzu4Fi: float
    fldfruPf8V9K6qIAN: str


type FormulasView = Literal[
    "Grid view",
]
"""View names for `Formulas`"""
FormulasViews: list[FormulasView] = [
    "Grid view",
]
"""View names for `Formulas`"""

type FormulasViewId = Literal[
    "viw7gdr4uJSpnHjR7",
]
"""View IDs for `Formulas`"""
FormulasViewIds: list[FormulasViewId] = [
    "viw7gdr4uJSpnHjR7",
]
"""View IDs for `Formulas`"""

FormulasViewNameIdMapping: dict[FormulasView, FormulasViewId] = {
    "Grid view": "viw7gdr4uJSpnHjR7",
}

FormulasViewIdNameMapping: dict[FormulasViewId, FormulasView] = {
    "viw7gdr4uJSpnHjR7": "Grid view",
}

# endregion

