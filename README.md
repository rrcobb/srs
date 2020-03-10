# A survey of spaced repetition algorithms in rust

(right now it's just sm2)

| Name  | Inputs  | Outputs  |
|---    |---      |---      |
| [SM2](#sm2)  | `quality`, `repetitions`, `previous ease factor`, `previous interval` | `interval`, `repetitions`, `ease factor` |
|       |         |         |
|       |         |         |

## SM2

## Other algorithms to implement

* Random chance re-encounter
* Leitner box
* Luhmann's Zettelkasten https://zettelkasten.de/posts/zettelkasten-improves-thinking-writing/
    * surprise and serendipity
    * relatedness
    * capturing things that you didn't realize or expect (should be a _real_ extension)
    * loose filing, interconnectedness - you have to forget about them so you can find them again. 
      * Admonishes against sorting and categorizing, favoring natural clusters around frequently noted topics
      * 'for every note, an id'
      * direct connections
* managing a graph-of-linked-notes
* fixed interval
* Pimsleur
* naive exponential
* mnemosyne - (basically sm2) https://github.com/mnemosyne-proj/mnemosyne/blob/master/mnemosyne/libmnemosyne/schedulers/SM2_mnemosyne.py
* neural network
* fibonacci, hilariously
    * https://www.quora.com/Whats-the-best-spaced-repetition-schedule
    * answerer actually does lay out a lot of the other possible sequences
* ebbinghaus forgetting curve
    * R = e ^ (-t/s), where R is recall, s is 'strength' of memory, t is time, e is euler's number.
    * s gets higher after subsequent review
    * original formula, for 'savings' as a percentage (with t in minutes): 100 * (1.84 / (log10(t)^1.25 + 1.84). Savings was defined as 'time saved from original learning time'
    * should show a lower bound for information for facts we are trying to remember (meaningless, disconnected)
    * nice formula elaboration https://psychology.stackexchange.com/a/5201
    * another stackexchange post suggests an alternate formula: R = a + (1-a) * b * (1 + t)^-beta
* More mathematical models of memory: https://psychology.stackexchange.com/questions/5772/what-are-the-mathematical-models-of-memory?rq=1
* sm-15 https://github.com/slaypni/SM-15
* halflife-regression from duolingo: https://github.com/duolingo/halflife-regression
* 'memorize', an optimized algorithm for spaced repetition: http://learning.mpi-sws.org/memorize/ and https://github.com/Networks-Learning/memorize

## Other areas of interest

* self-authored cards vs. other-authored cards
* learning to author cards well
* what kinds of inputs and data are easy / useful / possible
* kinds of cards, kinds of capture, kinds of interfaces
* curation vs. authoring
* SRS for many users
* overall system variables: fact introduction rate, card revision, time spent per day, overall error rate
* card feedback on prompt quality
* systems that promote usage frequency
* systems that are resilient to schedule-fall-apart
* context factors that affect retention in the short and long term (memory cues, prompts, and the environment)
    * prior knowledge
    * degree of importance
    * individual capability and context
    * general awareness / linkedness
    * review rate
    * frequency of retrieval
    * meaningfulness, stress, sleep
    * general mnemonic representation skills and techniques
* Interference from other knowledge / distractors / similar (but wrong) concepts
* Coherence with 'correct' prior knowledge
  * Cognates and false cognates
* multiple presentations in multiple forms (castles in the clounds / bootstrapping / skyhooks)
* fake social media feed as a srs presentation vehicle
* encoding errors are more likely when related facts are forgotten or misremembered
    * SRS is important because it helps the average recall strength for a particular fact
    * massed might be important if the subsequent 'fact network' is being built immediately
    * There are strong graph effects of related cards
* Some facts don't seem to suffer from forgetting. Why? Is it per person/fact pair, or are there generalities here?
* Remembering and getting it right feels good! What level of 'right' and 'wrong' should we aim for for getting the habits firmly in place?
* For 'natural' SRS systems (note reviewing, social media, environmental cues), how do we recognize that the world will provide us with chances to remember
    * how can we capture those in a computer-data driven srs?
    * how can we set those kinds of systems up for optimality?
* how does SRS relate to the Getting Shit Done / inbox zero / trusting your own capture mechanisms / limiting work in progress ideas?
* what are the neural substrates of remembering and forgetting? https://psychology.stackexchange.com/questions/9595/what-are-the-neural-substrates-of-retrieval-induced-forgetting
    * are there real RIFo effects?
* Nice discussion of some of the factors in memory models: https://www.reddit.com/r/Anki/comments/alhjgv/anyone_know_of_any_nonsm2_nonleitnerbased_srs/
* Mess: https://www.ribbonfarm.com/2017/01/05/tendrils-of-mess-in-our-brains/

### Links

* [Spaced repetition on Wikipedia](https://en.wikipedia.org/wiki/Spaced_repetition)
* [Official algorithm description](http://www.supermemo.com/english/ol/sm2.htm)

* Anki Simulator: https://pheartheceal.github.io/anki-optimizer/
* 
