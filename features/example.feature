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

  Scenario: A tuple with w=1.0 is a point
    Given a <- tuple 4.3, -4.2, 3.1, 1.0
    Then a.x = 4.3
    And a.z = 3.1
    And a.z = 3.1
    And a.w = 1.0
    And a is a point
    And a is not a vector

  Scenario: A tuple with w=0.0 is a vector
    Given a <- tuple 4.3, -4.2, 3.1, 0.0
    Then a.x = 4.3
    And a.z = 3.1
    And a.z = 3.1
    And a.w = 0.0
    And a is not a point
    And a is a vector

  Scenario: point factory creates tuples with w=1.0
  Given p <- point 4.0, -4.0, 3.0
  Then p = tuple 4.0, -4.0, 3.0, 1.0
  And p is a point

  Scenario: vector factory creates tuples with w=0.0
  Given p <- vector 4.0, -4.0, 3.0
  Then p = tuple 4.0, -4.0, 3.0, 0.0
  And p is a vector