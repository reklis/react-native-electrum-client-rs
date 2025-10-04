#import "ElectrumClient.h"
#import <React/RCTLog.h>

// C FFI declarations
extern char* electrum_start(const char* config_json);
extern char* electrum_stop(const char* network);
extern char* electrum_ping_server(const char* network);
extern char* electrum_get_header(const char* network, uint32_t height);
extern char* electrum_get_balance(const char* network, const char* script_hashes_json);
extern void electrum_free_string(char* s);

@implementation ElectrumClient

RCT_EXPORT_MODULE();

RCT_EXPORT_METHOD(start:(NSDictionary *)config
                  resolver:(RCTPromiseResolveBlock)resolve
                  rejecter:(RCTPromiseRejectBlock)reject)
{
    @try {
        NSError *error;
        NSData *jsonData = [NSJSONSerialization dataWithJSONObject:config options:0 error:&error];
        if (error) {
            reject(@"ELECTRUM_ERROR", @"Failed to serialize config", error);
            return;
        }

        NSString *jsonString = [[NSString alloc] initWithData:jsonData encoding:NSUTF8StringEncoding];
        const char *configCStr = [jsonString UTF8String];

        char *resultCStr = electrum_start(configCStr);
        NSString *resultString = [NSString stringWithUTF8String:resultCStr];
        electrum_free_string(resultCStr);

        NSData *resultData = [resultString dataUsingEncoding:NSUTF8StringEncoding];
        NSDictionary *result = [NSJSONSerialization JSONObjectWithData:resultData options:0 error:&error];

        if (error) {
            reject(@"ELECTRUM_ERROR", @"Failed to parse result", error);
            return;
        }

        resolve(result);
    } @catch (NSException *exception) {
        reject(@"ELECTRUM_ERROR", exception.reason, nil);
    }
}

RCT_EXPORT_METHOD(stop:(NSString *)network
                  resolver:(RCTPromiseResolveBlock)resolve
                  rejecter:(RCTPromiseRejectBlock)reject)
{
    @try {
        const char *networkCStr = [network UTF8String];

        char *resultCStr = electrum_stop(networkCStr);
        NSString *resultString = [NSString stringWithUTF8String:resultCStr];
        electrum_free_string(resultCStr);

        NSError *error;
        NSData *resultData = [resultString dataUsingEncoding:NSUTF8StringEncoding];
        NSDictionary *result = [NSJSONSerialization JSONObjectWithData:resultData options:0 error:&error];

        if (error) {
            reject(@"ELECTRUM_ERROR", @"Failed to parse result", error);
            return;
        }

        resolve(result);
    } @catch (NSException *exception) {
        reject(@"ELECTRUM_ERROR", exception.reason, nil);
    }
}

RCT_EXPORT_METHOD(pingServer:(NSString *)network
                  resolver:(RCTPromiseResolveBlock)resolve
                  rejecter:(RCTPromiseRejectBlock)reject)
{
    @try {
        const char *networkCStr = [network UTF8String];

        char *resultCStr = electrum_ping_server(networkCStr);
        NSString *resultString = [NSString stringWithUTF8String:resultCStr];
        electrum_free_string(resultCStr);

        NSError *error;
        NSData *resultData = [resultString dataUsingEncoding:NSUTF8StringEncoding];
        NSDictionary *result = [NSJSONSerialization JSONObjectWithData:resultData options:0 error:&error];

        if (error) {
            reject(@"ELECTRUM_ERROR", @"Failed to parse result", error);
            return;
        }

        resolve(result);
    } @catch (NSException *exception) {
        reject(@"ELECTRUM_ERROR", exception.reason, nil);
    }
}

RCT_EXPORT_METHOD(getHeader:(NSString *)network
                  height:(NSInteger)height
                  resolver:(RCTPromiseResolveBlock)resolve
                  rejecter:(RCTPromiseRejectBlock)reject)
{
    @try {
        const char *networkCStr = [network UTF8String];

        char *resultCStr = electrum_get_header(networkCStr, (uint32_t)height);
        NSString *resultString = [NSString stringWithUTF8String:resultCStr];
        electrum_free_string(resultCStr);

        NSError *error;
        NSData *resultData = [resultString dataUsingEncoding:NSUTF8StringEncoding];
        NSDictionary *result = [NSJSONSerialization JSONObjectWithData:resultData options:0 error:&error];

        if (error) {
            reject(@"ELECTRUM_ERROR", @"Failed to parse result", error);
            return;
        }

        resolve(result);
    } @catch (NSException *exception) {
        reject(@"ELECTRUM_ERROR", exception.reason, nil);
    }
}

RCT_EXPORT_METHOD(getBalance:(NSString *)network
                  scriptHashes:(NSArray *)scriptHashes
                  resolver:(RCTPromiseResolveBlock)resolve
                  rejecter:(RCTPromiseRejectBlock)reject)
{
    @try {
        NSError *error;
        NSData *jsonData = [NSJSONSerialization dataWithJSONObject:scriptHashes options:0 error:&error];
        if (error) {
            reject(@"ELECTRUM_ERROR", @"Failed to serialize script hashes", error);
            return;
        }

        NSString *jsonString = [[NSString alloc] initWithData:jsonData encoding:NSUTF8StringEncoding];
        const char *networkCStr = [network UTF8String];
        const char *hashesCStr = [jsonString UTF8String];

        char *resultCStr = electrum_get_balance(networkCStr, hashesCStr);
        NSString *resultString = [NSString stringWithUTF8String:resultCStr];
        electrum_free_string(resultCStr);

        NSData *resultData = [resultString dataUsingEncoding:NSUTF8StringEncoding];
        NSDictionary *result = [NSJSONSerialization JSONObjectWithData:resultData options:0 error:&error];

        if (error) {
            reject(@"ELECTRUM_ERROR", @"Failed to parse result", error);
            return;
        }

        resolve(result);
    } @catch (NSException *exception) {
        reject(@"ELECTRUM_ERROR", exception.reason, nil);
    }
}

@end
