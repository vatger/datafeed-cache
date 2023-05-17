import * as dotenv from 'dotenv';

dotenv.config();

export const Config = {
    APP_HOST: process.env.APP_HOST ?? '0.0.0.0',
    APP_PORT: Number(process.env.APP_PORT),
    DATAFEED_REFRESH_INTERVAL_S: Number(
        process.env.DATAFEED_REFRESH_INTERVAL_S ?? '15'
    ),
};
