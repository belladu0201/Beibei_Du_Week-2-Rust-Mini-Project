# Here is Rust Mini-Project for Week 3 (Due January 8. 2022)


## Introduction
In this project, I am trying to read a csv file and calculate the mean of average wage for each country. It serves as of a partial task for my IDS721 Project 1. The Repo can be found in this link: https://github.com/nogibjj/Beibei_Du_IDS721_Projet1

In this dataset I get from Kaggle: https://www.kaggle.com/datasets/kabhishm/countries-by-average-wage, I aim to find the averge minimum wage among the population from different countries. As a new user to Rust, I am trying to apply my data science skills into Rust. There are a lot of new library and new syntax for me to digest. So the task is very easy, calculate the mean of each country for the average wage across years.


## Code Display
![Alt text](https://github.com/belladu0201/Images_Beibei/blob/main/Screen%20Shot%202023-02-05%20at%202.36.34%20AM.png)

I created one function, which is to specifically calculate the row mean (skipping the first element, which is the country name). And in the `main` function, I read the csv using the `csv::Reader`, which I got help from Copilot, and ChatGPT. 

The partial output should look like below: 
![Alt text](https://github.com/belladu0201/Images_Beibei/blob/main/Screen%20Shot%202023-02-05%20at%202.39.31%20AM.png)

## Insights
- I think maybe Python is more appropriate for doing data-science intensive work. Rust should be used in other ways, such as some system stuff.
- Trying to see if I can do leetcode easy questions using Rust (not sure if that's efficient way to learn Rust and prepare for interviews).
