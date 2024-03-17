import { DatafeedModel } from '../models/DatafeedModel';
import axios, { AxiosResponse } from 'axios';
import { VatsimStatusModel } from '../models/VatsimStatusModel';

type DatafeedStatus = {
    url: string | null;
    last_queried_url: Date | null;
    data: DatafeedModel | null;
    last_update_failed: boolean;
};

let datafeedStatus: DatafeedStatus = {
    url: null,
    last_queried_url: null,
    data: null,
    last_update_failed: false,
};

let same_timestamp_count = 0;

async function getDatafeedURL(): Promise<boolean> {
    // Check if we need to query URL again
    if (datafeedStatus.url != null) {
        return true;
    }

    console.log('Querying Status');

    try {
        const res: AxiosResponse = await axios.get(
            'https://status.vatsim.net/status.json'
        );
        const status: VatsimStatusModel = res.data as VatsimStatusModel;

        datafeedStatus.last_queried_url = new Date();
        datafeedStatus.url =
            status.data.v3[Math.floor(Math.random() * status.data.v3.length)];
        return true;
    } catch (e) {
        console.error('Failed to retrieve Datafeed URL!');
        return false;
    }
}

async function getDatafeed(): Promise<DatafeedModel | null> {
    if (!(await getDatafeedURL()) || datafeedStatus.url == null) {
        return null;
    }

    console.log(new Date().toISOString(), 'Updating Datafeed');

    let res: AxiosResponse | undefined = undefined;
    try {
        res = await axios.get(datafeedStatus.url);

        // Check for valid JSON
        JSON.parse(JSON.stringify((<DatafeedModel>res?.data).general));
    } catch (e: any) {
        console.error('Failed to update Datafeed: ', e.message);
        datafeedStatus.last_update_failed = true;
        return null;
    }

    // Check if response looks good
    if (res == null || res.data == null) {
        return null;
    }

    const df = res.data as DatafeedModel;

    if (datafeedStatus.data?.general.update_timestamp == df.general.update_timestamp) {
        same_timestamp_count++;
    } else {
        same_timestamp_count = 0;
    }

    // Check if both update timestamps are equal -> No update
    // The chance that zero pilots are online is minuscule, such that we can conclude a failed datafeed update if there are no pilots
    // Also, if we queried multiple times and the same timestamp was found 6 times, then we can be pretty sure that the datafeed failed
    if (
        (datafeedStatus.data != null &&
            (Math.abs(datafeedStatus.data.pilots.length - df.pilots.length) > 75) || df.pilots.length == 0 || same_timestamp_count > 5)
    ) {
        console.error(
            '\t Update failed! Previous Pilot Count: ',
            datafeedStatus.data?.pilots.length,
            ' | Current Pilot Count: ',
            df.pilots.length,
            ' | Same Timestamp count: ',
            same_timestamp_count
        );
        datafeedStatus.last_update_failed = true;
        return datafeedStatus.data;
    }

    // Set values if all looks fine
    datafeedStatus.data = df;
    datafeedStatus.last_update_failed = false;

    console.log(
        `\t Done: ${datafeedStatus.data.controllers.length} Controllers, ${datafeedStatus.data.pilots.length} Pilots, Timestamp: ${datafeedStatus.data.general.update_timestamp} (Same Timestamp: ${same_timestamp_count})`
    );

    return datafeedStatus.data;
}

async function getCachedDatafeed(): Promise<DatafeedModel | null> {
    // Check if cached value is empty
    if (datafeedStatus.data != null) {
        return datafeedStatus.data;
    }

    return await getDatafeed();
}

function getUpdateFailed(): boolean {
    return datafeedStatus.last_update_failed;
}

export default {
    getDatafeed,
    getCachedDatafeed,
    getUpdateFailed,
};
