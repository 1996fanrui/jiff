#![allow(warnings)]

/*!
TODO

# Precise details of format

Here is a grammar of the supported format for "friendly" time durations.

```text
format =
    format-hms
    | format-designator

format-hms =
    sign? hours ':' minutes ':' seconds fractional?

format-designator =
    sign? format-designator-units
    | format-designator-units direction?
format-designator-units =
    years
    | months
    | weeks
    | days
    | hours
    | minutes
    | seconds
    | milliseconds
    | microseconds
    | nanoseconds

# This dance below is basically to ensure two things:
# First, that at least one unit appears. That is, that
# we don't accept the empty string. Secondly, when a
# fractional component appears in a time value, we don't
# allow any subsequent units to appear.
years =
    unit-value unit-years ws* months
    | unit-value unit-years ws* weeks
    | unit-value unit-years ws* days
    | unit-value unit-years ws* hours
    | unit-value unit-years ws* minutes
    | unit-value unit-years ws* seconds
    | unit-value unit-years ws* milliseconds
    | unit-value unit-years ws* microseconds
    | unit-value unit-years ws* nanoseconds
    | unit-value unit-years
months =
    unit-value unit-months ws* weeks
    | unit-value unit-months ws* days
    | unit-value unit-months ws* hours
    | unit-value unit-months ws* minutes
    | unit-value unit-months ws* seconds
    | unit-value unit-months ws* milliseconds
    | unit-value unit-months ws* microseconds
    | unit-value unit-months ws* nanoseconds
    | unit-value unit-months
weeks =
    unit-value unit-weeks ws* days
    | unit-value unit-weeks ws* hours
    | unit-value unit-weeks ws* minutes
    | unit-value unit-weeks ws* seconds
    | unit-value unit-weeks ws* milliseconds
    | unit-value unit-weeks ws* microseconds
    | unit-value unit-weeks ws* nanoseconds
    | unit-value unit-weeks
days =
    unit-value unit-days ws* hours
    | unit-value unit-days ws* minutes
    | unit-value unit-days ws* seconds
    | unit-value unit-days ws* milliseconds
    | unit-value unit-days ws* microseconds
    | unit-value unit-days ws* nanoseconds
    | unit-value unit-days
hours =
    unit-value unit-hours ws* minutes
    | unit-value unit-hours ws* seconds
    | unit-value unit-hours ws* milliseconds
    | unit-value unit-hours ws* microseconds
    | unit-value unit-hours ws* nanoseconds
    | unit-value fractional? ws* unit-hours
minutes =
    unit-value unit-minutes ws* seconds
    | unit-value unit-minutes ws* milliseconds
    | unit-value unit-minutes ws* microseconds
    | unit-value unit-minutes ws* nanoseconds
    | unit-value fractional? ws* unit-minutes
seconds =
    unit-value unit-seconds ws* milliseconds
    | unit-value unit-seconds ws* microseconds
    | unit-value unit-seconds ws* nanoseconds
    | unit-value fractional? ws* unit-seconds
milliseconds =
    unit-value unit-milliseconds ws* microseconds
    | unit-value unit-milliseconds ws* nanoseconds
    | unit-value fractional? ws* unit-milliseconds
microseconds =
    unit-value unit-microseconds ws* nanoseconds
    | unit-value fractional? ws* unit-microseconds
nanoseconds =
    unit-value fractional? ws* unit-nanoseconds

unit-value = [0-9]+ [ws*]
unit-years = 'years' | 'year' | 'y'
unit-months = 'months' | 'month' | 'M'
unit-weeks = 'weeks' | 'week' | 'w'
unit-days = 'days' | 'day' | 'd'
unit-hours = 'hours' | 'hour' | 'hrs' | 'hr' | 'h'
unit-minutes = 'minutes' | 'minute' | 'mins' | 'min' | 'm'
unit-seconds = 'seconds' | 'second' | 'secs' | 'sec' | 's'
unit-milliseconds =
    'milliseconds' | 'millisecond' | 'millis' | 'milli' | 'msec' | 'ms'
unit-microseconds =
    'microseconds'
    | 'microsecond'
    | 'micros'
    | 'micro'
    | 'usec'
    | 'us'
    | 'µ' (U+00B5 MICRO SIGN) 's'
unit-nanoseconds =
    'nanoseconds' | 'nanosecond' | 'nanos' | 'nano' | 'nsec' | 'ns'

fractional = decimal-separator decimal-fraction
decimal-separator = '.' | ','
decimal-fraction = [0-9]{1,9}

sign = '+' | '-'
direction = 'ago'
ws =
    U+0020 SPACE
    | U+0009 HORIZONTAL TAB
    | U+000A LINE FEED
    | U+000C FORM FEED
    | U+000D CARRIAGE RETURN
```
*/

/// TODO
#[derive(Debug)]
pub enum Descriptor {
    /// TODO
    Verbose,
    /// TODO
    Short,
    /// TODO
    Compact,
}

/// TODO
#[derive(Debug)]
pub enum Spacing {
    /// TODO
    None,
    /// TODO
    BetweenUnits,
    /// TODO
    BetweenUnitsAndDesignators,
}

/// TODO
#[derive(Debug)]
pub enum Direction {
    /// TODO
    Auto,
    /// TODO
    Sign,
    /// TODO
    Suffix,
}

/// TODO
#[derive(Debug)]
pub struct SpanPrinter {
    descriptor: Descriptor,
    spacing: Spacing,
    direction: Direction,
    fractional: bool,
    hms: bool,
}

impl SpanPrinter {
    /// TODO
    pub const fn new() -> SpanPrinter {
        SpanPrinter {
            descriptor: Descriptor::Short,
            spacing: Spacing::BetweenUnits,
            direction: Direction::Auto,
            fractional: false,
            hms: false,
        }
    }

    /// TODO
    pub const fn descriptor(self, descriptor: Descriptor) -> SpanPrinter {
        SpanPrinter { descriptor, ..self }
    }

    /// TODO
    pub const fn spacing(self, spacing: Spacing) -> SpanPrinter {
        SpanPrinter { spacing, ..self }
    }

    /// TODO
    pub const fn direction(self, direction: Direction) -> SpanPrinter {
        SpanPrinter { direction, ..self }
    }

    /// TODO
    pub const fn fractional(self, yes: bool) -> SpanPrinter {
        SpanPrinter { fractional: yes, ..self }
    }

    /// TODO
    pub const fn hours_minutes_seconds(self, yes: bool) -> SpanPrinter {
        SpanPrinter { hms: yes, ..self }
    }
}
