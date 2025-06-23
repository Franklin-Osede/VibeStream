export class DateRange {
  private constructor(
    private readonly startDate: Date,
    private readonly endDate: Date
  ) {
    if (startDate >= endDate) {
      throw new Error('Start date must be before end date');
    }
  }

  static create(startDate: Date, endDate: Date): DateRange {
    return new DateRange(startDate, endDate);
  }

  static fromStrings(startDateStr: string, endDateStr: string): DateRange {
    const start = new Date(startDateStr);
    const end = new Date(endDateStr);
    
    if (isNaN(start.getTime()) || isNaN(end.getTime())) {
      throw new Error('Invalid date format');
    }
    
    return new DateRange(start, end);
  }

  get start(): Date {
    return new Date(this.startDate);
  }

  get end(): Date {
    return new Date(this.endDate);
  }

  isValid(): boolean {
    return this.startDate < this.endDate;
  }

  isCurrentlyActive(): boolean {
    const now = new Date();
    return now >= this.startDate && now <= this.endDate;
  }

  startsInFuture(): boolean {
    return this.startDate > new Date();
  }

  isExpired(): boolean {
    return new Date() > this.endDate;
  }

  daysUntilStart(): number {
    const now = new Date();
    if (now >= this.startDate) return 0;
    
    const diffTime = this.startDate.getTime() - now.getTime();
    return Math.ceil(diffTime / (1000 * 60 * 60 * 24));
  }

  daysUntilEnd(): number {
    const now = new Date();
    if (now >= this.endDate) return 0;
    
    const diffTime = this.endDate.getTime() - now.getTime();
    return Math.ceil(diffTime / (1000 * 60 * 60 * 24));
  }

  durationInDays(): number {
    const diffTime = this.endDate.getTime() - this.startDate.getTime();
    return Math.ceil(diffTime / (1000 * 60 * 60 * 24));
  }

  totalDays(): number {
    return this.durationInDays();
  }

  contains(date: Date): boolean {
    return date >= this.startDate && date <= this.endDate;
  }

  overlaps(other: DateRange): boolean {
    return this.startDate <= other.endDate && this.endDate >= other.startDate;
  }

  equals(other: DateRange): boolean {
    return this.startDate.getTime() === other.startDate.getTime() &&
           this.endDate.getTime() === other.endDate.getTime();
  }

  toString(): string {
    return `${this.startDate.toISOString()} - ${this.endDate.toISOString()}`;
  }

  toJSON() {
    return {
      startDate: this.startDate.toISOString(),
      endDate: this.endDate.toISOString()
    };
  }
} 