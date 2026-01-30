/**
 * JavaScript 测试用例: ES6+ 特性
 */

// 类定义
class Animal {
  constructor(name) {
    this.name = name;
  }

  speak() {
    console.log(`${this.name} makes a sound.`);
  }

  static create(name) {
    return new Animal(name);
  }
}

// 继承
class Dog extends Animal {
  constructor(name, breed) {
    super(name);
    this.breed = breed;
  }

  speak() {
    console.log(`${this.name} barks.`);
  }

  fetch() {
    console.log(`${this.name} fetches the ball.`);
  }
}

// 函数声明
function greet(name) {
  return `Hello, ${name}!`;
}

// 箭头函数
const add = (a, b) => a + b;

const multiply = (a, b) => {
  return a * b;
};

// 异步函数
async function fetchData(url) {
  const response = await fetch(url);
  return response.json();
}

// 生成器
function* numberGenerator() {
  yield 1;
  yield 2;
  yield 3;
}

// 立即执行函数
const result = (function () {
  return "IIFE result";
})();

// 对象方法简写
const utils = {
  formatDate(date) {
    return date.toISOString();
  },
  parseDate(str) {
    return new Date(str);
  },
};

// 导出
export function exportedFunction() {
  return "exported";
}

export class ExportedClass {
  method() {
    return "method";
  }
}

export default Animal;
