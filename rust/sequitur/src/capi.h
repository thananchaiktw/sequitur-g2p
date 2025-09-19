#ifndef CAPI_H
#define CAPI_H

#ifdef __cplusplus
extern "C" {
#endif

    // Opaque structs
    typedef struct MultigramInventory_t MultigramInventory_t;
    typedef struct SequenceModel_t SequenceModel_t;
    typedef struct EstimationGraphBuilder_t EstimationGraphBuilder_t;
    typedef struct Translator_t Translator_t;

    // MultigramInventory
    MultigramInventory_t* MultigramInventory_new();
    void MultigramInventory_delete(MultigramInventory_t*);
    int MultigramInventory_size(MultigramInventory_t*);

    // SequenceModel
    SequenceModel_t* SequenceModel_new();
    void SequenceModel_delete(SequenceModel_t*);

    // EstimationGraphBuilder
    EstimationGraphBuilder_t* EstimationGraphBuilder_new();
    void EstimationGraphBuilder_delete(EstimationGraphBuilder_t*);
    void EstimationGraphBuilder_setSequenceModel(EstimationGraphBuilder_t*, MultigramInventory_t*, SequenceModel_t*);

    // Translator
    Translator_t* Translator_new();
    void Translator_delete(Translator_t*);
    void Translator_setMultigramInventory(Translator_t*, MultigramInventory_t*);
    void Translator_setSequenceModel(Translator_t*, SequenceModel_t*);
    int* Translator_translate(Translator_t*, const int*, int, int*);
    void free_sequence(int*);

#ifdef __cplusplus
}
#endif

#endif // CAPI_H
