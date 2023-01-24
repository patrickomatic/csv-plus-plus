# frozen_string_literal: true

require 'entities'

describe ::CSVPlusPlus::Language::Entities::Function do
  subject(:function) { described_class.new('FOO', %w[a b], build(:number_one)) }

  describe '#initialize' do
    it 'lowercases and converts the id to a symbol' do
      expect(subject.id).to(eq(:foo))
    end
  end

  describe '#function?' do
    it { is_expected.to(be_function) }
  end

  describe '#missingmethod' do
    it 'raises an error when called with a missing method' do
      expect { subject.missingmethod }
        .to(raise_error(::NoMethodError))
    end
  end

  describe '#==' do
    it { is_expected.to(eq(build(:fn, name: 'FOO', arguments: %w[a b], body: build(:number_one)))) }

    it { is_expected.not_to(eq(build(:fn_foo))) }
    it { is_expected.not_to(eq(build(:number_one))) }
    it { is_expected.not_to(eq(build(:variable_foo))) }
  end

  describe '#to_s' do
    subject { function.to_s }

    it { is_expected.to(eq('def FOO(a, b) 1')) }
  end
end
