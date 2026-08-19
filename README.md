![Screen Shot 2022-10-08 at 7 24 45 PM](https://user-images.githubusercontent.com/10949560/194731058-21ec39ac-7e6d-41ae-a192-9a894b1ceff1.png)
# MEGL Polyforms: Rust Version

Here is an accelerated version of what was originally prototyped in the Colab, with a few improvements

Tried multiple different algorithms to check validity as well as compare the speed using p values. All tested algorithms are in the code but earlyStoppingConditionPriorityQueue works twice as fast as the DFS the team used to be using.
Also starting out with a compact shape ensures that shape is closer to uniform distribution

Quality of life:
- Live rendering
- Easy CLI configuration

What if we had one thread find all contiguous blocks on the left side of center, another thread find all contiguous blocks right of center, and then see if each contiguous block to the left is contiguous with a block on the right at the end. 

to run use
cargo run --bin main -- --length [LENGTH] --export [EXPORT] --shuffles [SHUFFLES] --amount 1

<img width="2560" height="1080" alt="image" src="https://github.com/user-attachments/assets/161b322e-d5bc-45e1-8336-d76707380edd" />


**My Contribution**

I doubled the rate of which the core shuffle algorithm worked while also ensuring it had the same output. However it still is an O(n^4) algorithm per sample, which obviously scales poorly but that is the nature of the project. I had ideas to lower it to an O(n^3.5) but at the rate of 1/1,000,000 chance at a false negative, which the team decided was not worth it. I do not believe this would have a significant impact on the statistical distribution, however, my team disagreed. The project ended before I could find a way to test my theory.
