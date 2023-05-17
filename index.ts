import express, {Express} from 'express';
import { GlobalRouter } from './routes';
import DatafeedService from './service/DatafeedService';
import { Config } from './config';

const application: Express = express();

setInterval(async () => {
    await DatafeedService.getDatafeed();
}, 1000 * Config.DATAFEED_REFRESH_INTERVAL_S);

application.listen(Config.APP_PORT, Config.APP_HOST, () => {
    console.log(`Listening on ${Config.APP_HOST}:${Config.APP_PORT}`);
});

application.use('/', GlobalRouter);
