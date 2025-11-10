import sklearn_crfsuite
import collections


def main():
    crf = sklearn_crfsuite.CRF(model_filename="./dados/tagger.crf")
    features = collections.Counter(crf.state_features_).most_common()
    with open("./dados/params.txt", "w") as f:
        for (attr, label), weight in features:
            f.write("%0.6f %-8s %s\n" % (weight, label, attr))


if __name__ == "__main__":
    main()
