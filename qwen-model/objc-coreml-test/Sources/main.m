#import <Foundation/Foundation.h>
#import <CoreML/CoreML.h>

int main(int argc, const char * argv[]) {
    @autoreleasepool {
        NSString *modelPath = @"/Users/mazdahewitt/projects/candle-coreml/qwen-model/qwen_embeddings.mlmodelc";
        NSURL *modelURL = [NSURL fileURLWithPath:modelPath];
        
        NSError *error = nil;
        MLModel *model = [MLModel modelWithContentsOfURL:modelURL error:&error];
        if (error) {
            NSLog(@"Model loading failed: %@", error);
            return 1;
        }
        
        MLMultiArray *inputArray = [[MLMultiArray alloc] initWithShape:@[@1, @1]
                                                             dataType:MLMultiArrayDataTypeInt32
                                                                error:&error];
        if (error) {
            NSLog(@"Input creation failed: %@", error);
            return 1;
        }
        
        [inputArray setObject:@(1) atIndexedSubscript:0];
        
        NSDictionary *inputDict = @{@"input_ids": [MLFeatureValue featureValueWithMultiArray:inputArray]};
        MLDictionaryFeatureProvider *provider = [[MLDictionaryFeatureProvider alloc] 
                                                initWithDictionary:inputDict 
                                                           error:&error];
        if (error) {
            NSLog(@"Provider creation failed: %@", error);
            return 1;
        }
        
        id<MLFeatureProvider> prediction = [model predictionFromFeatures:provider 
                                                                  error:&error];
        if (error) {
            NSLog(@"Prediction failed: %@", error);
            return 1;
        }

        MLFeatureValue *hiddenStatesValue = [prediction featureValueForName:@"hidden_states"];
        MLMultiArray *embeddings = [hiddenStatesValue multiArrayValue];
        
        NSLog(@"Input: [1]");
        NSLog(@"Output shape: %@", embeddings.shape);
        
        float max_val = -INFINITY;
        float min_val = INFINITY;
        float sum = 0.0;
        
        for (NSUInteger i = 0; i < 10; i++) {
            float val = [embeddings[i] floatValue];
            NSLog(@"Value[%lu]: %f", i, val);
            max_val = MAX(max_val, val);
            min_val = MIN(min_val, val);
            sum += val;
        }
        
        NSLog(@"Max: %f", max_val);
        NSLog(@"Min: %f", min_val);
        NSLog(@"Mean: %f", sum / 10.0);
        
        return 0;
    }
}