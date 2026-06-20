#include "../../../Multigram.hh"
#include "../../../SequenceModel.hh"
#include "../../../Estimation.hh"
#include "../../../Translation.hh"

extern "C" {
    // Forward declarations
    struct MultigramInventory_t;
    struct SequenceModel_t;
    struct EstimationGraphBuilder_t;
    struct Translator_t;

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


    // Implementations
    struct MultigramInventory_t {
        MultigramInventory *rep;
    };

    MultigramInventory_t* MultigramInventory_new() {
        return new MultigramInventory_t{ new MultigramInventory() };
    }

    void MultigramInventory_delete(MultigramInventory_t* p) {
        delete p->rep;
        delete p;
    }

    int MultigramInventory_size(MultigramInventory_t* p) {
        return p->rep->size();
    }

    struct SequenceModel_t {
        SequenceModel *rep;
    };

    SequenceModel_t* SequenceModel_new() {
        return new SequenceModel_t{ new SequenceModel() };
    }

    void SequenceModel_delete(SequenceModel_t* p) {
        delete p->rep;
        delete p;
    }

    struct EstimationGraphBuilder_t {
        EstimationGraphBuilder *rep;
    };

    EstimationGraphBuilder_t* EstimationGraphBuilder_new() {
        return new EstimationGraphBuilder_t{ new EstimationGraphBuilder() };
    }

    void EstimationGraphBuilder_delete(EstimationGraphBuilder_t* p) {
        delete p->rep;
        delete p;
    }

    void EstimationGraphBuilder_setSequenceModel(EstimationGraphBuilder_t* p, MultigramInventory_t* inv, SequenceModel_t* model) {
        p->rep->setSequenceModel(inv->rep, model->rep);
    }

    struct Translator_t {
        Translator *rep;
    };

    Translator_t* Translator_new() {
        return new Translator_t{ new Translator() };
    }

    void Translator_delete(Translator_t* p) {
        delete p->rep;
        delete p;
    }

    void Translator_setMultigramInventory(Translator_t* p, MultigramInventory_t* inv) {
        p->rep->setMultigramInventory(inv->rep);
    }

    void Translator_setSequenceModel(Translator_t* p, SequenceModel_t* model) {
        p->rep->setSequenceModel(model->rep);
    }

    int* Translator_translate(Translator_t* p, const int* in_seq, int in_len, int* out_len) {
        Sequence in;
        for (int i = 0; i < in_len; ++i) {
            in.push_back(in_seq[i]);
        }

        std::vector<MultigramIndex> out_mgs;
        p->rep->translate(in, out_mgs);

        *out_len = out_mgs.size();
        int* out_seq = new int[*out_len];
        for (size_t i = 0; i < *out_len; ++i) {
            out_seq[i] = out_mgs[i];
        }
        return out_seq;
    }

    void free_sequence(int* p) {
        delete[] p;
    }
}
