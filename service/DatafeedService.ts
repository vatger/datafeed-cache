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

    console.log(new Date(), 'Updating Datafeed');

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

    // Check if both update timestamps are equal -> No update
    // The chance that zero pilots are online is minuscule, such that we can conclude a failed datafeed update if there are no pilots
    if (
        (datafeedStatus.data != null &&
            Math.abs(
                datafeedStatus.data.pilots.length -
                    (<DatafeedModel>res.data).pilots.length
            ) > 75) ||
        (<DatafeedModel>res.data).pilots.length == 0
    ) {
        console.error(
            '\t Update failed! Previous Pilot Count: ',
            datafeedStatus.data?.pilots.length,
            ' | Current Pilot Count: ',
            (<DatafeedModel>res.data).pilots.length
        );
        datafeedStatus.last_update_failed = true;
        return datafeedStatus.data;
    }

    // Set values if all looks fine
    datafeedStatus.data = res.data as DatafeedModel;
    datafeedStatus.last_update_failed = false;

    console.log(
        `\t Done: ${datafeedStatus.data.controllers.length} Controllers, ${datafeedStatus.data.pilots.length} Pilots`
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
