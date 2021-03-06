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

  Scenario: Adding two tuples
  Given a <- tuple 3.0, -2.0, 5.0, 1.0
  And b <- tuple -2.0, 3.0, 1.0, 0.0
  Then a + b == tuple 1.0, 1.0, 6.0, 1.0

  Scenario: Subtracting two points
  Given p1 <- point 3.0, 2.0, 1.0
  And p2 <- point 5.0, 6.0, 7.0
  Then p1 - p2 == vector -2.0, -4.0, -6.0

  Scenario: Subtracting a vector from a point
  Given p <- point 3.0, 2.0, 1.0
  And v <- vector 5.0, 6.0, 7.0
  Then p - v == point -2.0, -4.0, -6.0

  Scenario: Subtracting two vectors
  Given v1 <- vector 3.0, 2.0, 1.0
  And v2 <- vector 5.0, 6.0, 7.0
  Then v1 - v2 == vector -2.0, -4.0, -6.0

  Scenario: Subtracting a vector from the zero vector
  Given zero <- vector 0.0, 0.0, 0.0
  And v <- vector 1.0, -2.0, 3.0
  Then zero - v == vector -1.0, 2.0, -3.0

  Scenario: Negating a tuple
  Given a <- tuple 1.0, -2.0, 3.0, -4.0
  Then neg a == tuple -1.0, 2.0, -3.0, 4.0

  Scenario: Multiplying a tuple by a scalar
  Given a <- tuple 1.0, -2.0, 3.0, -4.0
  Then a * 3.5 == tuple 3.5, -7.0, 10.5, -14.0

  Scenario: Multiplying a tuple by a fraction
  Given a <- tuple 1.0, -2.0, 3.0, -4.0
  Then a * 0.5 == tuple 0.5, -1.0, 1.5, -2.0

  Scenario: Dividing a tuple by a scalar
  Given a <- tuple 1.0, -2.0, 3.0, -4.0
  Then a / 2.0 == tuple 0.5, -1.0, 1.5, -2.0

  Scenario: Computing the magnitude of vector(1,0,0)
  Given v <- vector 1.0, 0.0, 0.0
  Then magnitude v == 1.0

  Scenario: Computing the magnitude of vector(0,1,0)
  Given v <- vector 0.0, 1.0, 0.0
  Then magnitude v == 1.0

  Scenario: Computing the magnitude of vector(0,0,1)
  Given v <- vector 0.0, 0.0, 1.0
  Then magnitude v == 1.0

  Scenario: Computing the magnitude of vector(1,2,3)
  Given v <- vector 1.0, 2.0, 3.0
  Then magnitude v == sqrt 14.0

  Scenario: Computing the magnitude of vector(-1,-2,-3)
  Given v <- vector 1.0, 2.0, 3.0
  Then magnitude v == sqrt 14.0

  Scenario: Normalizing vector(4,0,0) gives vector(1,0,0)
  Given v <- vector 4.0, 0.0, 0.0
  Then normalize v == vector 1.0, 0.0, 0.0

  Scenario: Normalizing vector(1,2,3)
  Given v <- vector 1.0, 2.0, 3.0
  Then normalize v == approximately vector 0.26726, 0.53452, 0.80178

  Scenario: Magnitude of normalize vector
  Given v <- vector 1.0, 2.0, 3.0
  When norm <- normalize v
  Then magnitude norm == 1.0

  Scenario: The dot product of two tuples
  Given a <- vector 1.0, 2.0, 3.0
  And b <- vector 2.0, 3.0, 4.0
  Then dot a, b == 20.0

  Scenario: The cross porduct of two vectors
  Given a <- vector 1.0, 2.0, 3.0
  And b <- vector 2.0, 3.0, 4.0
  Then cross a, b == vector -1.0, 2.0, -1.0
  And cross b, a == vector 1.0, -2.0, 1.0