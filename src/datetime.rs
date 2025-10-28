use std::time::{SystemTime, UNIX_EPOCH};

const SECONDS_PER_HOUR: u64 = 3600;

#[derive(Debug)]
pub struct AxDateTime {
    pub year: u64,
    pub month: u64,
    pub day: u64,
    pub hour: u64,
    pub minute: u64,
    pub second: u64
}

impl AxDateTime {
    /// Создает объект даты и времени из временной метки Unix и смещения UTC
    pub fn from_timestamp(timestamp: u64, utc_offset_hours: i64) -> Result<Self, String> {
        let seconds_offset = utc_offset_hours * SECONDS_PER_HOUR as i64;
        let adjusted_timestamp = timestamp as i64 + seconds_offset;

        if adjusted_timestamp < 0 {
            return Err(format!("Некорректная временная метка после учета смещения UTC."));
        }

        let mut remaining_seconds = adjusted_timestamp as u64;

        let second = remaining_seconds % 60;
        remaining_seconds /= 60;

        let minute = remaining_seconds % 60;
        remaining_seconds /= 60;

        let hour = remaining_seconds % 24;
        remaining_seconds /= 24;

        // Количество полных дней с момента Unix epoch
        let total_days = remaining_seconds;

        // Выбираем начальный год и считаем полный остаток дней
        let mut year = 1970;
        let mut days_passed = 0;

        while days_passed <= total_days {
            let days_in_current_year = if Self::is_leap_year(year) { 366 } else { 365 };
            if days_passed + days_in_current_year > total_days {
                break;
            }
            days_passed += days_in_current_year;
            year += 1;
        }

        // Оставшиеся дни после последнего целого года
        let remaining_days_in_year = total_days - days_passed;

        // Определяем месяц и день
        let mut month = 1;
        let mut days_left = remaining_days_in_year;

        while days_left >= Self::days_in_month(month, year)? {
            days_left -= Self::days_in_month(month, year)?;
            month += 1;
        }

        let day = days_left + 1;

        Ok(Self {
            second,
            minute,
            hour,
            day,
            month,
            year,
        })
    }

    /// Создает объект даты и времени из текущей метки Unix и смещения UTC
    pub fn now(utc_offset_hours: i64) -> Result<Self, String> {
        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH).map_err(|err| err.to_string())?;
        Self::from_timestamp(since_the_epoch.as_secs(), utc_offset_hours)
    }

    /// Определяет, является ли заданный год високосным.
    ///
    /// # Аргументы
    ///
    /// * `year`: Год, проверяемый на високосность.
    ///
    /// # Возвращаемое значение
    ///
    /// * `true`, если год високосный, иначе `false`.
    pub fn is_leap_year(year: u64) -> bool {
        // Проверяем три правила високосного года:
        // Деление на 4, кроме тех годов, которые делятся на 100, но не делятся на 400.
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    /// Подсчет дней в указанном месяце
    pub fn days_in_month(month: u64, year: u64) -> Result<u64, String>  {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => Ok(31),
            4 | 6 | 9 | 11 => Ok(30),
            2 => {
                if Self::is_leap_year(year) {
                    Ok(29)
                } else {
                    Ok(28)
                }
            }
            _ => Err(format!("Неверный номер месяца")),
        }
    }
}