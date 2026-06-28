# ==========================================
# Auto-generated file. Do not edit directly.
# ==========================================

from datetime import datetime
from typing import cast
from pyairtable.orm import Model
from pyairtable.orm.fields import (
    SingleLineTextField,
    DateField,
    NumberField,
    DatetimeField,
)
from ...static.helpers import (
    get_api_key,
    get_base_id,
    build_url,
)
from ...static.airtable_runtime import AirtableRuntime as F
from ..dicts import FormulasRecordDict
from ..formulas import FormulasFormulas
from ..types import (
    FormulasView,
    FormulasViewNameIdMapping,
)
import urllib.parse
import math
import re


class FormulasModel(Model):
    """
    ORM model for Airtable records from the `Formulas` table.

    Property names do not necessarily match field names in Airtable.
    """
    class Meta:
        @staticmethod
        def api_key() -> str:
            return get_api_key()
        @staticmethod
        def base_id() -> str:
            return get_base_id()
        table_name = "Formulas"
        use_field_ids = True
        memoize = True

    def to_record_dict(self, only_writable: bool = False) -> FormulasRecordDict:
        return cast("FormulasRecordDict", self.to_record(only_writable))

    def url(self, view: FormulasView | None = None) -> str:
        """Get the URL for this record in Airtable, with optional view."""
        if view:
            return build_url(base_id=get_base_id(), table_id='tblnuYBsMdXNDsuRc', record_id=self.id, view_id=FormulasViewNameIdMapping[view])
        else:
            return build_url(base_id=get_base_id(), table_id='tblnuYBsMdXNDsuRc', record_id=self.id)

    f: FormulasFormulas = FormulasFormulas()

    evaluate_formulas_at_runtime: bool = False

    _orm_date_formula: SingleLineTextField = SingleLineTextField(field_name="fldY7kjaklLeoSgGd", readonly=True)
    @property
    def date_formula(self) -> str | None:
        """
        Date Formula `fldY7kjaklLeoSgGd` - `Read-Only Field`
        
        ```
        "YEAR: " & YEAR({First Date})
        & ", MONTH: " & MONTH({First Date})
        & ", DAY: " & DAY({First Date})
        & ", HOUR: " & HOUR({Second Date})
        & ", MINUTE: " & MINUTE({Second Date})
        & ", SECOND: " & SECOND({Second Date})
        & ", TODAY: " & DATETIME_FORMAT(
          TODAY(),
          'YYYY-MM-DD'
        )
        & ", WORKDAY+5: " & DATETIME_FORMAT(
          WORKDAY({First Date}, 5),
          'YYYY-MM-DD'
        )
        & ", WORKDAY_DIFF: " & WORKDAY_DIFF({First Date}, {Second Date})
        & ", PARSE: " & DATETIME_FORMAT(
          DATETIME_PARSE(
            DATESTR({First Date})
          ),
          'YYYY-MM-DD'
        )
        & ", FORMAT: " & DATETIME_FORMAT({Second Date}, 'YYYY-MM-DD HH:mm')
        & ", LOCALE: " & DATETIME_FORMAT(
          SET_LOCALE({Second Date}, 'en'),
          'YYYY-MM-DD'
        )
        & ", TIMEZONE: " & DATETIME_FORMAT(
          SET_TIMEZONE({Second Date}, 'America/New_York'),
          'YYYY-MM-DD HH:mm'
        )
        & ", DATESTR: " & DATESTR({Second Date})
        & ", TIMESTR: " & TIMESTR({Second Date})
        & ", TONOW: " & TONOW({First Date})
        & ", FROMNOW: " & FROMNOW({Second Date})
        & ", DATEADD+2d: " & DATETIME_FORMAT(
          DATEADD({First Date}, 2, 'days'),
          'YYYY-MM-DD'
        )
        & ", WEEKNUM: " & WEEKNUM({First Date}, 'Monday')
        & ", DIFF days: " & DATETIME_DIFF({First Date}, {Second Date}, 'days')
        & ", IS_BEFORE: " & IF(
          IS_BEFORE({First Date}, {Second Date}),
          'Yes',
          'No'
        )
        & ", IS_SAME day: " & IF(
          IS_SAME({First Date}, {Second Date}, 'day'),
          'Yes',
          'No'
        )
        & ", IS_AFTER: " & IF(
          IS_AFTER({Third Date}, {Second Date}),
          'Yes',
          'No'
        )
        ```
        """
        if self.evaluate_formulas_at_runtime:
            self._fields["fldY7kjaklLeoSgGd"] = ((((((((((((((((((((((((((((((((((((((((((((("YEAR: " + F.S(F.D(self.first_date).year)) + ", MONTH: ") + F.S(F.D(self.first_date).month)) + ", DAY: ") + F.S(F.D(self.first_date).day)) + ", HOUR: ") + F.S(F.D(self.second_date).hour)) + ", MINUTE: ") + F.S(F.D(self.second_date).minute)) + ", SECOND: ") + F.S(F.D(self.second_date).second)) + ", TODAY: ") + F.S(F.DATETIME_FORMAT(datetime.now().strftime("%Y-%m-%d"), 'YYYY-MM-DD'))) + ", WORKDAY+5: ") + F.S(F.DATETIME_FORMAT(F.WORKDAY(self.first_date, 5), 'YYYY-MM-DD'))) + ", WORKDAY_DIFF: ") + F.S(F.WORKDAY_DIFF(self.first_date, self.second_date))) + ", PARSE: ") + F.S(F.DATETIME_FORMAT(F.D(F.D(self.first_date).strftime("%Y-%m-%d")), 'YYYY-MM-DD'))) + ", FORMAT: ") + F.S(F.DATETIME_FORMAT(self.second_date, 'YYYY-MM-DD HH:mm'))) + ", LOCALE: ") + F.S(F.DATETIME_FORMAT(self.second_date, 'YYYY-MM-DD'))) + ", TIMEZONE: ") + F.S(F.DATETIME_FORMAT(F.SET_TIMEZONE(self.second_date, 'America/New_York'), 'YYYY-MM-DD HH:mm'))) + ", DATESTR: ") + F.S(F.D(self.second_date).strftime("%Y-%m-%d"))) + ", TIMESTR: ") + F.S(F.D(self.second_date).strftime("%H:%M:%S"))) + ", TONOW: ") + F.S(F.TONOW(self.first_date))) + ", FROMNOW: ") + F.S(F.FROMNOW(self.second_date))) + ", DATEADD+2d: ") + F.S(F.DATETIME_FORMAT(F.DATEADD(self.first_date, 2, 'days'), 'YYYY-MM-DD'))) + ", WEEKNUM: ") + F.S(F.WEEKNUM(self.first_date, 'Monday'))) + ", DIFF days: ") + F.S(F.DATETIME_DIFF(self.first_date, self.second_date, 'days'))) + ", IS_BEFORE: ") + F.S(('Yes' if F.IS_BEFORE(self.first_date, self.second_date) else 'No'))) + ", IS_SAME day: ") + F.S(('Yes' if F.IS_SAME(self.first_date, self.second_date, 'day') else 'No'))) + ", IS_AFTER: ") + F.S(('Yes' if F.IS_AFTER(self.third_date, self.second_date) else 'No')))
        return self._fields.get("fldY7kjaklLeoSgGd")
    first_date: DateField = DateField(field_name="fldlZT521Iy0FFXFL")
    """First Date `fldlZT521Iy0FFXFL`"""
    first_number: NumberField = NumberField(field_name="fldA04pqfjMkGXcZU")
    """First Number `fldA04pqfjMkGXcZU`"""
    first_text: SingleLineTextField = SingleLineTextField(field_name="fldHJuw6pAujnHvkP")
    """First Text `fldHJuw6pAujnHvkP`"""
    _orm_math_formula: SingleLineTextField = SingleLineTextField(field_name="fldlSuvoeWGokSz8Z", readonly=True)
    @property
    def math_formula(self) -> str | None:
        """
        Math Formula `fldlSuvoeWGokSz8Z` - `Read-Only Field`
        
        ```
        IF(
          OR(
            {First Number} = BLANK(),
            {Second Number} = BLANK()
          ),
          BLANK(),
          "SUM: " & SUM({First Number}, {Second Number})
          & ", MIN: " & MIN({First Number}, {Second Number})
          & ", MAX: " & MAX({First Number}, {Second Number})
          & ", AVG: " & AVERAGE({First Number}, {Second Number})
          & ", COUNT: " & COUNT({First Number}, {Second Number})
          & ", CEIL: " & CEILING({First Number})
          & ", FLOOR: " & FLOOR({Second Number})
          & ", ROUND: " & ROUND({First Number}/2, 1)
          & ", ROUNDUP: " & ROUNDUP({Second Number}/2, 0)
          & ", ROUNDDOWN: " & ROUNDDOWN({First Number}/2, 0)
          & ", INT: " & INT({Second Number}/2)
          & ", EVEN: " & EVEN({First Number})
          & ", ODD: " & ODD({Second Number})
          & ", MOD: " & MOD({First Number}, {Second Number})
          & ", LOG: " & LOG({First Number})
          & ", EXP: " & EXP(1)
          & ", POWER: " & POWER({First Number}, 2)
          & ", SQRT: " & SQRT(
            ABS({Second Number})
          ) & ", ABS: " & ABS({Second Number})
        )
        ```
        """
        if self.evaluate_formulas_at_runtime:
            self._fields["fldlSuvoeWGokSz8Z"] = (None if ((self.first_number == None) or (self.second_number == None)) else ((((((((((((((((((((((((((((((((((((("SUM: " + F.S(sum(F.AN((self.first_number, self.second_number,))))) + ", MIN: ") + F.S(min(F.AN((self.first_number, self.second_number,))))) + ", MAX: ") + F.S(max(F.AN((self.first_number, self.second_number,))))) + ", AVG: ") + F.S(F.AVERAGE(self.first_number, self.second_number))) + ", COUNT: ") + F.S(F.COUNT(self.first_number, self.second_number))) + ", CEIL: ") + F.S(F.CEILING(self.first_number))) + ", FLOOR: ") + F.S(F.FLOOR(self.second_number))) + ", ROUND: ") + F.S(round((F.N(self.first_number) / 2), 1))) + ", ROUNDUP: ") + F.S(F.ROUNDUP((F.N(self.second_number) / 2), 0))) + ", ROUNDDOWN: ") + F.S(F.ROUNDDOWN((F.N(self.first_number) / 2), 0))) + ", INT: ") + F.S(math.floor((F.N(self.second_number) / 2)))) + ", EVEN: ") + F.S(F.EVEN(self.first_number))) + ", ODD: ") + F.S(F.ODD(self.second_number))) + ", MOD: ") + F.S((F.N(self.first_number) % F.N(self.second_number)))) + ", LOG: ") + F.S(F.LOG(self.first_number))) + ", EXP: ") + F.S(math.exp(1))) + ", POWER: ") + F.S(math.pow(F.N(self.first_number), 2))) + ", SQRT: ") + F.S(math.sqrt(F.N(abs(F.N(self.second_number)))))) + ", ABS: ") + F.S(abs(F.N(self.second_number)))))
        return self._fields.get("fldlSuvoeWGokSz8Z")
    primary_key: SingleLineTextField = SingleLineTextField(field_name="fldLZFrZKvSCS4dKb")
    """Primary Key `fldLZFrZKvSCS4dKb` - `Primary Key`"""
    second_date: DatetimeField = DatetimeField(field_name="fld1LZ3Ebpt0LaMmu")
    """Second Date `fld1LZ3Ebpt0LaMmu`"""
    second_number: NumberField = NumberField(field_name="fldj5nAkal5y8OOZg")
    """Second Number `fldj5nAkal5y8OOZg`"""
    second_text: SingleLineTextField = SingleLineTextField(field_name="fldA2boNwwsiuvXw1")
    """Second Text `fldA2boNwwsiuvXw1`"""
    _orm_text_formula: SingleLineTextField = SingleLineTextField(field_name="flddvzeqt7FJpQ9NX", readonly=True)
    @property
    def text_formula(self) -> str | None:
        """
        Text Formula `flddvzeqt7FJpQ9NX` - `Read-Only Field`
        
        ```
        CONCATENATE(
          "LEN: ",
          LEN({First Text}),
          "; ",
          "MID: ",
          MID({First Text}, 2, 3),
          "; ",
          "LEFT: ",
          LEFT({Second Text}, 2),
          "; ",
          "RIGHT: ",
          RIGHT({Second Text}, 2),
          "; ",
          "FIND: ",
          FIND("e", {First Text}),
          "; ",
          "SEARCH: ",
          SEARCH("e", {First Text}),
          "; ",
          "REPLACE: ",
          REPLACE({First Text}, 2, 2, "XX"),
          "; ",
          "REPT: ",
          REPT({First Text}, 2),
          "; ",
          "LOWER: ",
          LOWER({Second Text}),
          "; ",
          "UPPER: ",
          UPPER({Second Text}),
          "; ",
          "TRIM: ",
          TRIM("   " & {First Text} & "   "),
          "; ",
          "SUBSTITUTE: ",
          SUBSTITUTE({First Text}, "e", "@"),
          "; ",
          "CONCATENATE: ",
          CONCATENATE(
            {First Text},
            "-",
            {Second Text},
            "-",
            {Third Text}
          ),
          "; ",
          "T: ",
          T({First Number}),
          "; ",
          "REGEX_EXTRACT: ",
          REGEX_EXTRACT({First Text}, "[aeiou]"),
          "; ",
          "REGEX_MATCH: ",
          IF(
            REGEX_MATCH({First Text}, "^.e"),
            "1",
            "0"
          ),
          "; ",
          "REGEX_REPLACE: ",
          REGEX_REPLACE({First Text}, "[aeiou]", "*"),
          "; ",
          "ENCODE_URL_COMPONENT: ",
          ENCODE_URL_COMPONENT({First Text})
        )
        ```
        """
        if self.evaluate_formulas_at_runtime:
            self._fields["flddvzeqt7FJpQ9NX"] = "".join(map(F.S, ("LEN: ", len(F.S(self.first_text)), "; ", "MID: ", F.MID(self.first_text, 2, 3), "; ", "LEFT: ", F.LEFT(self.second_text, 2), "; ", "RIGHT: ", F.RIGHT(self.second_text, 2), "; ", "FIND: ", F.FIND("e", self.first_text), "; ", "SEARCH: ", F.SEARCH("e", self.first_text), "; ", "REPLACE: ", F.REPLACE(self.first_text, 2, 2, "XX"), "; ", "REPT: ", (F.S(self.first_text) * 2), "; ", "LOWER: ", F.S(self.second_text).lower(), "; ", "UPPER: ", F.S(self.second_text).upper(), "; ", "TRIM: ", (("   " + F.S(self.first_text)) + "   ").strip(), "; ", "SUBSTITUTE: ", F.SUBSTITUTE(self.first_text, "e", "@"), "; ", "CONCATENATE: ", "".join(map(F.S, (self.first_text, "-", self.second_text, "-", self.third_text,))), "; ", "T: ", F.T(self.first_number), "; ", "REGEX_EXTRACT: ", (m.group(0) if (m := re.search("[aeiou]", F.S(self.first_text))) else None), "; ", "REGEX_MATCH: ", ("1" if bool(re.search("^.e", F.S(self.first_text))) else "0"), "; ", "REGEX_REPLACE: ", re.sub("[aeiou]", "*", F.S(self.first_text)), "; ", "ENCODE_URL_COMPONENT: ", urllib.parse.quote(F.S(self.first_text), safe="!*'()"),)))
        return self._fields.get("flddvzeqt7FJpQ9NX")
    third_date: DatetimeField = DatetimeField(field_name="fldxSQRRn8W879aiU")
    """Third Date `fldxSQRRn8W879aiU`"""
    third_number: NumberField = NumberField(field_name="fld5NBdekrAUzu4Fi")
    """Third Number `fld5NBdekrAUzu4Fi`"""
    third_text: SingleLineTextField = SingleLineTextField(field_name="fldfruPf8V9K6qIAN")
    """Third Text `fldfruPf8V9K6qIAN`"""

