Feature: Example feature

  Scenario: An example scenario
    Given I am trying out Cucumber
    When I consider what I am doing
    Then I am interested in ATDD
    And we can implement rules with regex

  Scenario: Concatening two arrays should create a new array
    Given a <- array3 1, 2, 3
    And b <- array3 3, 4, 5
    When c <- concat a,b
    Then c = array6 1, 2, 3, 3, 4, 5
