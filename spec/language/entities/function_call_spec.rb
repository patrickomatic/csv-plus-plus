# frozen_string_literal: true

require 'entities'

describe ::CSVPlusPlus::Language::Entities::FunctionCall do
  subject { described_class.new('MINUS', %w[a b]) }

  describe '#initialize' do
    it 'lowercases and converts the id to a symbol' do
      expect(subject.id).to(eq(:minus))
    end
  end

  describe '#function_call?' do
    it { is_expected.to(be_function_call) }
  end

  describe '#==' do
    it { is_expected.to(eq(build(:fn_call, name: 'minus', arguments: %w[a b]))) }

    it { is_expected.not_to(eq(build(:fn_foo))) }
    it { is_expected.not_to(eq(build(:number_one))) }
    it { is_expected.not_to(eq(build(:variable_foo))) }
  end
end
