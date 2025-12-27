import numpy as np
import pandas as pd

# Dow Jones historical daily prices
# Source:
# https://www.wsj.com/market-data/quotes/index/DJIA/historical-prices?gaa_at=eafs&gaa_n=AWEtsqeUx9OtIh1HPjSWpt7OTmEoNgZj_p9vRka6eZosXWlhzc6m-QguaMMAIcQYmYY%3D&gaa_ts=694e40bb&gaa_sig=F7PgnRwvwgCJ7NqozxmdlJ0wU3kpbreVBVrlXCzDKLD7puw-J4jIsMQRJToDlVRY7LkYa1GlxDyrThN-SBPOVQ%3D%3D
df = pd.read_csv("/Users/abdallahdar/Downloads/DowJones_Historical.csv")

# Add additional days of the year to this df that will experience trading:
extend = pd.DataFrame({"Date" : ["12/26/25","12/29/25","12/30/25","12/31/25"]})

# Combine tables and format date
df = pd.concat([df, extend])
df["date"] = pd.to_datetime(df["Date"], format = "%m/%d/%y")

all_dates1 = pd.date_range(
    start="1977-01-03",
    end="2025-12-31",
    freq="D"
)

trading_days = set(df["date"])

records = []
for d in all_dates1:
    date_str = d.strftime("%Y-%m-%d")
    weekday = d.weekday()  # 0=Mon … 6=Sun

    is_weekend = weekday >= 5
    is_trading = d in trading_days
    is_holiday = (not is_trading) and (not is_weekend)

    records.append({
        "date": date_str,
        "d": d,
        "trading": is_trading,
        "weekend": is_weekend,
        "holiday": is_holiday,
    })

# Future holidays
# Source: https://www.nyse.com/markets/hours-calendars
holidays_df = pd.DataFrame({"Date" : ["01/01/26","01/19/26","02/16/26","04/03/26","05/25/26",
                                   "06/19/26","07/03/26","09/07/26","11/26/26","12/25/26",
                                   "01/01/27","01/18/27","02/15/27","03/26/27","05/31/27",
                                   "06/18/27","07/05/27","09/06/27","11/25/27","12/24/27"]})
holidays_df["date"] = pd.to_datetime(holidays_df["Date"], format = "%m/%d/%y")

all_dates2 = pd.date_range(
    start="2026-01-01",
    end="2027-12-31",
    freq="D"
)

holidays = set(holidays_df["date"])

for d in all_dates2:
    date_str = d.strftime("%Y-%m-%d")
    weekday = d.weekday()  # 0=Mon … 6=Sun

    is_weekend = weekday >= 5
    is_holiday = d in holidays
    is_trading = (not is_holiday) and (not is_weekend)

    records.append({
        "date": date_str,
        "d": d,
        "trading": is_trading,
        "weekend": is_weekend,
        "holiday": is_holiday,
    })

# Convert records to df
out = pd.DataFrame(records).drop(columns = "d")
holidays_only = out[out["holiday"]][["date"]].sort_values("date")

# Save data
holidays_only.to_csv(
    "nyse_holidays.csv",
    index=False
)
out.to_csv(
    "nyse_calendar_full.csv",
    index=False
)