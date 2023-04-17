#ifndef _STRUCTS_H_
#define _STRUCTS_H_

struct A {
    int a, b, c, d;
};

struct B {
    long a, b, c, d;
};

struct C {
    float a, b, c, d;
};

struct D {
    double a, b, c, d;
};


// class MyClass
// {
// public:
//   int field;
//   virtual void method() const = 0;

//   static const int static_field;
//   static int static_method();
// };

#include <tuple>
class foo
{
	public:
        void method(int a, int b){}
		foo(int n_, char c_, double d_)
			: n{n_}, c{c_}, d{d_}
		{}
		friend bool operator<(const foo& lh, const foo& rh)
		{
			return std::tie(lh.n, lh.c, lh.d) <
			       std::tie(rh.n, rh.c, rh.d);
		}
	private:
		int n;
		char c;
		double d;
};


#endif