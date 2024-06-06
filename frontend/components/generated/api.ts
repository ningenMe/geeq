/* tslint:disable */
/* eslint-disable */
/**
 * geeq-api
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


import type { Configuration } from './configuration';
import type { AxiosPromise, AxiosInstance, RawAxiosRequestConfig } from 'axios';
import globalAxios from 'axios';
// Some imports not used depending on template conditions
// @ts-ignore
import { DUMMY_BASE_URL, assertParamExists, setApiKeyToObject, setBasicAuthToObject, setBearerAuthToObject, setOAuthToObject, setSearchParams, serializeDataIfNeeded, toPathString, createRequestFunction } from './common';
import type { RequestArgs } from './base';
// @ts-ignore
import { BASE_PATH, COLLECTION_FORMATS, BaseAPI, RequiredError, operationServerMap } from './base';

/**
 * 
 * @export
 * @interface AuthLoginPostRequest
 */
export interface AuthLoginPostRequest {
    /**
     * 
     * @type {string}
     * @memberof AuthLoginPostRequest
     */
    'code': string;
}
/**
 * 
 * @export
 * @interface AuthMeGet200Response
 */
export interface AuthMeGet200Response {
    /**
     * 
     * @type {User}
     * @memberof AuthMeGet200Response
     */
    'user': User;
}
/**
 * 
 * @export
 * @interface Common200Response
 */
export interface Common200Response {
    /**
     * 
     * @type {string}
     * @memberof Common200Response
     */
    'message': string;
}
/**
 * 
 * @export
 * @interface Common400Response
 */
export interface Common400Response {
    /**
     * 
     * @type {string}
     * @memberof Common400Response
     */
    'message': string;
}
/**
 * 
 * @export
 * @interface Common401Response
 */
export interface Common401Response {
    /**
     * 
     * @type {string}
     * @memberof Common401Response
     */
    'message': string;
}
/**
 * 
 * @export
 * @interface Common500Response
 */
export interface Common500Response {
    /**
     * 
     * @type {string}
     * @memberof Common500Response
     */
    'message': string;
}
/**
 * 
 * @export
 * @interface Task
 */
export interface Task {
    /**
     * 
     * @type {string}
     * @memberof Task
     */
    'taskId': string;
    /**
     * 
     * @type {string}
     * @memberof Task
     */
    'title': string;
    /**
     * 
     * @type {string}
     * @memberof Task
     */
    'description': string;
    /**
     * 
     * @type {string}
     * @memberof Task
     */
    'createdAt': string;
    /**
     * 
     * @type {string}
     * @memberof Task
     */
    'updatedAt': string;
    /**
     * userId
     * @type {string}
     * @memberof Task
     */
    'createdBy': string;
}
/**
 * 
 * @export
 * @interface TaskGet200Response
 */
export interface TaskGet200Response {
    /**
     * 
     * @type {Array<Task>}
     * @memberof TaskGet200Response
     */
    'tasks': Array<Task>;
}
/**
 * 
 * @export
 * @interface TaskTaskIdGet200Response
 */
export interface TaskTaskIdGet200Response {
    /**
     * 
     * @type {Task}
     * @memberof TaskTaskIdGet200Response
     */
    'task': Task;
}
/**
 * 
 * @export
 * @interface User
 */
export interface User {
    /**
     * 
     * @type {string}
     * @memberof User
     */
    'userId': string;
    /**
     * 
     * @type {string}
     * @memberof User
     */
    'avatarUrl': string;
}

/**
 * DefaultApi - axios parameter creator
 * @export
 */
export const DefaultApiAxiosParamCreator = function (configuration?: Configuration) {
    return {
        /**
         * 
         * @param {AuthLoginPostRequest} authLoginPostRequest 
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        authLoginPost: async (authLoginPostRequest: AuthLoginPostRequest, options: RawAxiosRequestConfig = {}): Promise<RequestArgs> => {
            // verify required parameter 'authLoginPostRequest' is not null or undefined
            assertParamExists('authLoginPost', 'authLoginPostRequest', authLoginPostRequest)
            const localVarPath = `/auth/login`;
            // use dummy base URL string because the URL constructor only accepts absolute URLs.
            const localVarUrlObj = new URL(localVarPath, DUMMY_BASE_URL);
            let baseOptions;
            if (configuration) {
                baseOptions = configuration.baseOptions;
            }

            const localVarRequestOptions = { method: 'POST', ...baseOptions, ...options};
            const localVarHeaderParameter = {} as any;
            const localVarQueryParameter = {} as any;


    
            localVarHeaderParameter['Content-Type'] = 'application/json';

            setSearchParams(localVarUrlObj, localVarQueryParameter);
            let headersFromBaseOptions = baseOptions && baseOptions.headers ? baseOptions.headers : {};
            localVarRequestOptions.headers = {...localVarHeaderParameter, ...headersFromBaseOptions, ...options.headers};
            localVarRequestOptions.data = serializeDataIfNeeded(authLoginPostRequest, localVarRequestOptions, configuration)

            return {
                url: toPathString(localVarUrlObj),
                options: localVarRequestOptions,
            };
        },
        /**
         * 
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        authLogoutPost: async (options: RawAxiosRequestConfig = {}): Promise<RequestArgs> => {
            const localVarPath = `/auth/logout`;
            // use dummy base URL string because the URL constructor only accepts absolute URLs.
            const localVarUrlObj = new URL(localVarPath, DUMMY_BASE_URL);
            let baseOptions;
            if (configuration) {
                baseOptions = configuration.baseOptions;
            }

            const localVarRequestOptions = { method: 'POST', ...baseOptions, ...options};
            const localVarHeaderParameter = {} as any;
            const localVarQueryParameter = {} as any;


    
            setSearchParams(localVarUrlObj, localVarQueryParameter);
            let headersFromBaseOptions = baseOptions && baseOptions.headers ? baseOptions.headers : {};
            localVarRequestOptions.headers = {...localVarHeaderParameter, ...headersFromBaseOptions, ...options.headers};

            return {
                url: toPathString(localVarUrlObj),
                options: localVarRequestOptions,
            };
        },
        /**
         * 
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        authMeGet: async (options: RawAxiosRequestConfig = {}): Promise<RequestArgs> => {
            const localVarPath = `/auth/me`;
            // use dummy base URL string because the URL constructor only accepts absolute URLs.
            const localVarUrlObj = new URL(localVarPath, DUMMY_BASE_URL);
            let baseOptions;
            if (configuration) {
                baseOptions = configuration.baseOptions;
            }

            const localVarRequestOptions = { method: 'GET', ...baseOptions, ...options};
            const localVarHeaderParameter = {} as any;
            const localVarQueryParameter = {} as any;


    
            setSearchParams(localVarUrlObj, localVarQueryParameter);
            let headersFromBaseOptions = baseOptions && baseOptions.headers ? baseOptions.headers : {};
            localVarRequestOptions.headers = {...localVarHeaderParameter, ...headersFromBaseOptions, ...options.headers};

            return {
                url: toPathString(localVarUrlObj),
                options: localVarRequestOptions,
            };
        },
        /**
         * 
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        taskGet: async (options: RawAxiosRequestConfig = {}): Promise<RequestArgs> => {
            const localVarPath = `/task`;
            // use dummy base URL string because the URL constructor only accepts absolute URLs.
            const localVarUrlObj = new URL(localVarPath, DUMMY_BASE_URL);
            let baseOptions;
            if (configuration) {
                baseOptions = configuration.baseOptions;
            }

            const localVarRequestOptions = { method: 'GET', ...baseOptions, ...options};
            const localVarHeaderParameter = {} as any;
            const localVarQueryParameter = {} as any;


    
            setSearchParams(localVarUrlObj, localVarQueryParameter);
            let headersFromBaseOptions = baseOptions && baseOptions.headers ? baseOptions.headers : {};
            localVarRequestOptions.headers = {...localVarHeaderParameter, ...headersFromBaseOptions, ...options.headers};

            return {
                url: toPathString(localVarUrlObj),
                options: localVarRequestOptions,
            };
        },
        /**
         * 
         * @param {string} taskId 
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        taskTaskIdGet: async (taskId: string, options: RawAxiosRequestConfig = {}): Promise<RequestArgs> => {
            // verify required parameter 'taskId' is not null or undefined
            assertParamExists('taskTaskIdGet', 'taskId', taskId)
            const localVarPath = `/task/{taskId}`
                .replace(`{${"taskId"}}`, encodeURIComponent(String(taskId)));
            // use dummy base URL string because the URL constructor only accepts absolute URLs.
            const localVarUrlObj = new URL(localVarPath, DUMMY_BASE_URL);
            let baseOptions;
            if (configuration) {
                baseOptions = configuration.baseOptions;
            }

            const localVarRequestOptions = { method: 'GET', ...baseOptions, ...options};
            const localVarHeaderParameter = {} as any;
            const localVarQueryParameter = {} as any;


    
            setSearchParams(localVarUrlObj, localVarQueryParameter);
            let headersFromBaseOptions = baseOptions && baseOptions.headers ? baseOptions.headers : {};
            localVarRequestOptions.headers = {...localVarHeaderParameter, ...headersFromBaseOptions, ...options.headers};

            return {
                url: toPathString(localVarUrlObj),
                options: localVarRequestOptions,
            };
        },
    }
};

/**
 * DefaultApi - functional programming interface
 * @export
 */
export const DefaultApiFp = function(configuration?: Configuration) {
    const localVarAxiosParamCreator = DefaultApiAxiosParamCreator(configuration)
    return {
        /**
         * 
         * @param {AuthLoginPostRequest} authLoginPostRequest 
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        async authLoginPost(authLoginPostRequest: AuthLoginPostRequest, options?: RawAxiosRequestConfig): Promise<(axios?: AxiosInstance, basePath?: string) => AxiosPromise<Common200Response>> {
            const localVarAxiosArgs = await localVarAxiosParamCreator.authLoginPost(authLoginPostRequest, options);
            const localVarOperationServerIndex = configuration?.serverIndex ?? 0;
            const localVarOperationServerBasePath = operationServerMap['DefaultApi.authLoginPost']?.[localVarOperationServerIndex]?.url;
            return (axios, basePath) => createRequestFunction(localVarAxiosArgs, globalAxios, BASE_PATH, configuration)(axios, localVarOperationServerBasePath || basePath);
        },
        /**
         * 
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        async authLogoutPost(options?: RawAxiosRequestConfig): Promise<(axios?: AxiosInstance, basePath?: string) => AxiosPromise<Common200Response>> {
            const localVarAxiosArgs = await localVarAxiosParamCreator.authLogoutPost(options);
            const localVarOperationServerIndex = configuration?.serverIndex ?? 0;
            const localVarOperationServerBasePath = operationServerMap['DefaultApi.authLogoutPost']?.[localVarOperationServerIndex]?.url;
            return (axios, basePath) => createRequestFunction(localVarAxiosArgs, globalAxios, BASE_PATH, configuration)(axios, localVarOperationServerBasePath || basePath);
        },
        /**
         * 
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        async authMeGet(options?: RawAxiosRequestConfig): Promise<(axios?: AxiosInstance, basePath?: string) => AxiosPromise<AuthMeGet200Response>> {
            const localVarAxiosArgs = await localVarAxiosParamCreator.authMeGet(options);
            const localVarOperationServerIndex = configuration?.serverIndex ?? 0;
            const localVarOperationServerBasePath = operationServerMap['DefaultApi.authMeGet']?.[localVarOperationServerIndex]?.url;
            return (axios, basePath) => createRequestFunction(localVarAxiosArgs, globalAxios, BASE_PATH, configuration)(axios, localVarOperationServerBasePath || basePath);
        },
        /**
         * 
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        async taskGet(options?: RawAxiosRequestConfig): Promise<(axios?: AxiosInstance, basePath?: string) => AxiosPromise<TaskGet200Response>> {
            const localVarAxiosArgs = await localVarAxiosParamCreator.taskGet(options);
            const localVarOperationServerIndex = configuration?.serverIndex ?? 0;
            const localVarOperationServerBasePath = operationServerMap['DefaultApi.taskGet']?.[localVarOperationServerIndex]?.url;
            return (axios, basePath) => createRequestFunction(localVarAxiosArgs, globalAxios, BASE_PATH, configuration)(axios, localVarOperationServerBasePath || basePath);
        },
        /**
         * 
         * @param {string} taskId 
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        async taskTaskIdGet(taskId: string, options?: RawAxiosRequestConfig): Promise<(axios?: AxiosInstance, basePath?: string) => AxiosPromise<TaskTaskIdGet200Response>> {
            const localVarAxiosArgs = await localVarAxiosParamCreator.taskTaskIdGet(taskId, options);
            const localVarOperationServerIndex = configuration?.serverIndex ?? 0;
            const localVarOperationServerBasePath = operationServerMap['DefaultApi.taskTaskIdGet']?.[localVarOperationServerIndex]?.url;
            return (axios, basePath) => createRequestFunction(localVarAxiosArgs, globalAxios, BASE_PATH, configuration)(axios, localVarOperationServerBasePath || basePath);
        },
    }
};

/**
 * DefaultApi - factory interface
 * @export
 */
export const DefaultApiFactory = function (configuration?: Configuration, basePath?: string, axios?: AxiosInstance) {
    const localVarFp = DefaultApiFp(configuration)
    return {
        /**
         * 
         * @param {AuthLoginPostRequest} authLoginPostRequest 
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        authLoginPost(authLoginPostRequest: AuthLoginPostRequest, options?: any): AxiosPromise<Common200Response> {
            return localVarFp.authLoginPost(authLoginPostRequest, options).then((request) => request(axios, basePath));
        },
        /**
         * 
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        authLogoutPost(options?: any): AxiosPromise<Common200Response> {
            return localVarFp.authLogoutPost(options).then((request) => request(axios, basePath));
        },
        /**
         * 
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        authMeGet(options?: any): AxiosPromise<AuthMeGet200Response> {
            return localVarFp.authMeGet(options).then((request) => request(axios, basePath));
        },
        /**
         * 
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        taskGet(options?: any): AxiosPromise<TaskGet200Response> {
            return localVarFp.taskGet(options).then((request) => request(axios, basePath));
        },
        /**
         * 
         * @param {string} taskId 
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        taskTaskIdGet(taskId: string, options?: any): AxiosPromise<TaskTaskIdGet200Response> {
            return localVarFp.taskTaskIdGet(taskId, options).then((request) => request(axios, basePath));
        },
    };
};

/**
 * DefaultApi - object-oriented interface
 * @export
 * @class DefaultApi
 * @extends {BaseAPI}
 */
export class DefaultApi extends BaseAPI {
    /**
     * 
     * @param {AuthLoginPostRequest} authLoginPostRequest 
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof DefaultApi
     */
    public authLoginPost(authLoginPostRequest: AuthLoginPostRequest, options?: RawAxiosRequestConfig) {
        return DefaultApiFp(this.configuration).authLoginPost(authLoginPostRequest, options).then((request) => request(this.axios, this.basePath));
    }

    /**
     * 
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof DefaultApi
     */
    public authLogoutPost(options?: RawAxiosRequestConfig) {
        return DefaultApiFp(this.configuration).authLogoutPost(options).then((request) => request(this.axios, this.basePath));
    }

    /**
     * 
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof DefaultApi
     */
    public authMeGet(options?: RawAxiosRequestConfig) {
        return DefaultApiFp(this.configuration).authMeGet(options).then((request) => request(this.axios, this.basePath));
    }

    /**
     * 
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof DefaultApi
     */
    public taskGet(options?: RawAxiosRequestConfig) {
        return DefaultApiFp(this.configuration).taskGet(options).then((request) => request(this.axios, this.basePath));
    }

    /**
     * 
     * @param {string} taskId 
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof DefaultApi
     */
    public taskTaskIdGet(taskId: string, options?: RawAxiosRequestConfig) {
        return DefaultApiFp(this.configuration).taskTaskIdGet(taskId, options).then((request) => request(this.axios, this.basePath));
    }
}



