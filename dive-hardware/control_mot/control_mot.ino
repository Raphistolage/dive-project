#include <Servo.h>

Servo myservo1;
Servo myservo2;

int val; 
int i = 0;
int pos = 0;

const int vibrationMotorPin = 8;

void setup() {
  myservo1.attach(10);
  myservo1.write(0);
  myservo2.attach(9);
  myservo2.write(180);
  pinMode(vibrationMotorPin, OUTPUT);
  digitalWrite(vibrationMotorPin, LOW);  // Motor is off at begining.
  Serial.begin(9600);
}

void loop() {
  if (Serial.available() >= 2) {
    int command = Serial.read();
    if (command == 9) {
      int contraction = Serial.read();
      val = map(contraction, 0, 100, 0, 180);     // scale it to use it with the servo (value between 0 and 180)
      myservo1.write(val);
      myservo2.write(180-val);
      delay(15);
      /*
      if(contraction > 50){
        myservo.write(180); 
        delay(200);
        myservo.write(90);
      } else {
        myservo.write(0);
        delay(200); 
        myservo.write(90);
      }
      */
    }
    else if (command == 8){
      digitalWrite(vibrationMotorPin, HIGH);  // Motor on
      delay(500);                    // Vibrate for 500ms
      digitalWrite(vibrationMotorPin, LOW);   // Motor off
    }
  }

  
}
