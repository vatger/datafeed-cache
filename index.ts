import express from 'express';
import { GlobalRouter } from './routes';
import DatafeedService from './service/DatafeedService';
import { Config } from './config';

const application = express();

setInterval(async () => {
    await DatafeedService.getDatafeed();
}, 1000 * Config.DATAFEED_REFRESH_INTERVAL_S);

application.listen(Config.APP_PORT, Config.APP_HOST, () => {
    console.log(`Listening on port: ${Config.APP_PORT}`);
});

application.use('/', GlobalRouter);
