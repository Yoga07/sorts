package com.sample.test;

import java.util.*;

public class Main{
  static{
    System.loadLibrary("sorts");
  }

  public static void main(String[] args) {
    System.out.print("Enter the Length of the array:");
    Scanner sc = new Scanner(System.in);
    int n = sc.nextInt();
    int array[] = new int[n];
    System.out.print("Enter the elements:");
    for(int i = 0;i<n;i++){
      array[i] = sc.nextInt();
    }
    System.out.print("Enter the Sort:\n1. Quick sort\n2. Selection sort\n3. Bucket sort\n");
    int c = sc.nextInt();
    Library.Initiator(array,c);
  }
}
