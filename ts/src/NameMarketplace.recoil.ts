/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.19.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { selectorFamily } from "recoil";
import { cosmWasmClient } from "./chain";
import { Uint128, InstantiateMsg, ExecuteMsg, Timestamp, Uint64, QueryMsg, Addr, BidOffset, AskResponse, Ask, AskCountResponse, HooksResponse, AsksResponse, BidResponse, Bid, BidsResponse, ConfigResponse, Decimal, ParamsResponse, SudoParams } from "./NameMarketplace.types";
import { NameMarketplaceQueryClient } from "./NameMarketplace.client";
type QueryClientParams = {
  contractAddress: string;
};
export const queryClient = selectorFamily<NameMarketplaceQueryClient, QueryClientParams>({
  key: "nameMarketplaceQueryClient",
  get: ({
    contractAddress
  }) => ({
    get
  }) => {
    const client = get(cosmWasmClient);
    return new NameMarketplaceQueryClient(client, contractAddress);
  }
});
export const askSelector = selectorFamily<AskResponse, QueryClientParams & {
  params: Parameters<NameMarketplaceQueryClient["ask"]>;
}>({
  key: "nameMarketplaceAsk",
  get: ({
    params,
    ...queryClientParams
  }) => async ({
    get
  }) => {
    const client = get(queryClient(queryClientParams));
    return await client.ask(...params);
  }
});
export const asksSelector = selectorFamily<AsksResponse, QueryClientParams & {
  params: Parameters<NameMarketplaceQueryClient["asks"]>;
}>({
  key: "nameMarketplaceAsks",
  get: ({
    params,
    ...queryClientParams
  }) => async ({
    get
  }) => {
    const client = get(queryClient(queryClientParams));
    return await client.asks(...params);
  }
});
export const reverseAsksSelector = selectorFamily<AsksResponse, QueryClientParams & {
  params: Parameters<NameMarketplaceQueryClient["reverseAsks"]>;
}>({
  key: "nameMarketplaceReverseAsks",
  get: ({
    params,
    ...queryClientParams
  }) => async ({
    get
  }) => {
    const client = get(queryClient(queryClientParams));
    return await client.reverseAsks(...params);
  }
});
export const askCountSelector = selectorFamily<AskCountResponse, QueryClientParams & {
  params: Parameters<NameMarketplaceQueryClient["askCount"]>;
}>({
  key: "nameMarketplaceAskCount",
  get: ({
    params,
    ...queryClientParams
  }) => async ({
    get
  }) => {
    const client = get(queryClient(queryClientParams));
    return await client.askCount(...params);
  }
});
export const asksBySellerSelector = selectorFamily<AsksResponse, QueryClientParams & {
  params: Parameters<NameMarketplaceQueryClient["asksBySeller"]>;
}>({
  key: "nameMarketplaceAsksBySeller",
  get: ({
    params,
    ...queryClientParams
  }) => async ({
    get
  }) => {
    const client = get(queryClient(queryClientParams));
    return await client.asksBySeller(...params);
  }
});
export const bidSelector = selectorFamily<BidResponse, QueryClientParams & {
  params: Parameters<NameMarketplaceQueryClient["bid"]>;
}>({
  key: "nameMarketplaceBid",
  get: ({
    params,
    ...queryClientParams
  }) => async ({
    get
  }) => {
    const client = get(queryClient(queryClientParams));
    return await client.bid(...params);
  }
});
export const bidsByBidderSelector = selectorFamily<BidsResponse, QueryClientParams & {
  params: Parameters<NameMarketplaceQueryClient["bidsByBidder"]>;
}>({
  key: "nameMarketplaceBidsByBidder",
  get: ({
    params,
    ...queryClientParams
  }) => async ({
    get
  }) => {
    const client = get(queryClient(queryClientParams));
    return await client.bidsByBidder(...params);
  }
});
export const bidsSelector = selectorFamily<BidsResponse, QueryClientParams & {
  params: Parameters<NameMarketplaceQueryClient["bids"]>;
}>({
  key: "nameMarketplaceBids",
  get: ({
    params,
    ...queryClientParams
  }) => async ({
    get
  }) => {
    const client = get(queryClient(queryClientParams));
    return await client.bids(...params);
  }
});
export const bidsSortedByPriceSelector = selectorFamily<BidsResponse, QueryClientParams & {
  params: Parameters<NameMarketplaceQueryClient["bidsSortedByPrice"]>;
}>({
  key: "nameMarketplaceBidsSortedByPrice",
  get: ({
    params,
    ...queryClientParams
  }) => async ({
    get
  }) => {
    const client = get(queryClient(queryClientParams));
    return await client.bidsSortedByPrice(...params);
  }
});
export const reverseBidsSortedByPriceSelector = selectorFamily<BidsResponse, QueryClientParams & {
  params: Parameters<NameMarketplaceQueryClient["reverseBidsSortedByPrice"]>;
}>({
  key: "nameMarketplaceReverseBidsSortedByPrice",
  get: ({
    params,
    ...queryClientParams
  }) => async ({
    get
  }) => {
    const client = get(queryClient(queryClientParams));
    return await client.reverseBidsSortedByPrice(...params);
  }
});
export const highestBidSelector = selectorFamily<BidResponse, QueryClientParams & {
  params: Parameters<NameMarketplaceQueryClient["highestBid"]>;
}>({
  key: "nameMarketplaceHighestBid",
  get: ({
    params,
    ...queryClientParams
  }) => async ({
    get
  }) => {
    const client = get(queryClient(queryClientParams));
    return await client.highestBid(...params);
  }
});
export const askHooksSelector = selectorFamily<HooksResponse, QueryClientParams & {
  params: Parameters<NameMarketplaceQueryClient["askHooks"]>;
}>({
  key: "nameMarketplaceAskHooks",
  get: ({
    params,
    ...queryClientParams
  }) => async ({
    get
  }) => {
    const client = get(queryClient(queryClientParams));
    return await client.askHooks(...params);
  }
});
export const bidHooksSelector = selectorFamily<HooksResponse, QueryClientParams & {
  params: Parameters<NameMarketplaceQueryClient["bidHooks"]>;
}>({
  key: "nameMarketplaceBidHooks",
  get: ({
    params,
    ...queryClientParams
  }) => async ({
    get
  }) => {
    const client = get(queryClient(queryClientParams));
    return await client.bidHooks(...params);
  }
});
export const saleHooksSelector = selectorFamily<HooksResponse, QueryClientParams & {
  params: Parameters<NameMarketplaceQueryClient["saleHooks"]>;
}>({
  key: "nameMarketplaceSaleHooks",
  get: ({
    params,
    ...queryClientParams
  }) => async ({
    get
  }) => {
    const client = get(queryClient(queryClientParams));
    return await client.saleHooks(...params);
  }
});
export const paramsSelector = selectorFamily<ParamsResponse, QueryClientParams & {
  params: Parameters<NameMarketplaceQueryClient["params"]>;
}>({
  key: "nameMarketplaceParams",
  get: ({
    params,
    ...queryClientParams
  }) => async ({
    get
  }) => {
    const client = get(queryClient(queryClientParams));
    return await client.params(...params);
  }
});
export const renewalQueueSelector = selectorFamily<AsksResponse, QueryClientParams & {
  params: Parameters<NameMarketplaceQueryClient["renewalQueue"]>;
}>({
  key: "nameMarketplaceRenewalQueue",
  get: ({
    params,
    ...queryClientParams
  }) => async ({
    get
  }) => {
    const client = get(queryClient(queryClientParams));
    return await client.renewalQueue(...params);
  }
});
export const configSelector = selectorFamily<ConfigResponse, QueryClientParams & {
  params: Parameters<NameMarketplaceQueryClient["config"]>;
}>({
  key: "nameMarketplaceConfig",
  get: ({
    params,
    ...queryClientParams
  }) => async ({
    get
  }) => {
    const client = get(queryClient(queryClientParams));
    return await client.config(...params);
  }
});